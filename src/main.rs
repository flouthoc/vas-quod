extern crate getopts;
extern crate simple_error;
use getopts::Options;
use simple_error::require_with;
use std::cmp::min;
use std::env;
use std::error::Error;
use std::process;

mod runtime;
mod cgroup;
mod filesystem;
mod mount;
mod namespace;

fn print_usage(program: &str, opts: &Options) {
	let brief = format!("Usage: {} vas-quod [options] [-- <command> <argument>...]", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let mut opts = Options::new();
	opts.optopt("r", "rootfs", "Path to root file-system eg. --rootfs /home/alpinefs", "path");
	opts.optopt("c", "command", "Command to be executed eg. --command `curl http://google.com`", "command");
	opts.optflag("h", "help", "Print this help menu");

	// Find the conventional "--" that separates out remaing arguments.
	let end_processable = args.iter().position(|s| s == "--").unwrap_or_else(|| args.len());
	let begin_unprocessable = min(end_processable + 1, args.len());

	let matches = opts.parse(&args[1..end_processable]).ok().unwrap_or_else(|| {
		println!("Error: Unrecognzied options");
		print_usage(&program, &opts);
		process::exit(7);
	});

	// Exits early, but doesn't lead to non-zero exit.
	if matches.opt_present("h") {
		print_usage(&program, &opts);
		return;
	}

	let rootfs = matches.opt_str("r").unwrap_or_else(|| {
		println!("Error: Please pass `--rootfs <path>`");
		print_usage(&program, &opts);
		process::exit(7);
	});

	let c = matches.opt_str("c"); // NB: Seperate let binding for lifetime.
	let (command, args) = determine_command_tuple(&c, &args[begin_unprocessable..args.len()]).ok().unwrap_or_else(|| {
		println!("Error: Please pass `--command <shell command>` or `-- <command> <argument>...`");
		print_usage(&program, &opts);
		process::exit(7);
	});

	runtime::run_container(&rootfs, command, args);
}

/// Determines based on the inputs whether we are going to invoke a shell, with
/// shell interpretation, or a simple unescaped argument vector.
/// Rearranges arguments as needed, but doesn't reallocate.
fn determine_command_tuple<'a, T: AsRef<str> + 'a>(shell_command: &'a Option<T>, argv: &'a [T]) -> Result<(&'a str, Vec<&'a str>), Box<dyn Error>> {
	let mut vec: Vec<&str> = vec![];

	// Prepend shell and command string if a command string is given.
	if let Some(shell_command) = shell_command {
		vec.push("/bin/sh");
		vec.push("-c");
		vec.push(shell_command.as_ref());
	}

	// The args given will be the whole command if there's no shell string;
	// otherwise, they'll be added to the argument vector.
	vec.extend(argv.iter().map(|item| item.as_ref()));

	// Shift off the first word as the command, erroring out if no command is
	// given.
	vec.reverse();
	let command = require_with!(vec.pop(), "Empty command!");
	vec.reverse();

	return Ok((command, vec));
}
