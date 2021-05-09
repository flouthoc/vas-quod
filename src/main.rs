extern crate getopts;
extern crate simple_error;
extern crate clap;

use clap::{Arg, App};
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
	let _program = args[0].clone();
	let command_vec: Vec<&str>;
	let rootfs: &str;

	let mut app = App::new("vas-quod")
		.version("1.0")
		.about("Linux Container runtime")
		.arg(Arg::new("command")
			.short('c')
			.long("command")
			.value_name("command")
			.about("command to be executed eg. --command 'curl http://google.com'")
			.min_values(1)
			.takes_value(true))
		.arg(Arg::new("rootfs")
			.short('r')
			.long("rootfs")
			.value_name("rootfs")
			.about("Path to root file-system eg. --rootfs /home/alpinefs")
			.takes_value(true))
		.subcommand(App::new("spec")
			.about("create a new specification file")
			.arg(Arg::new("rootless")
				.about("Generate a rootless spec"))
			.arg(Arg::new("bundle")
				.about("Path to bundle"))
		);

	let matches = app.get_matches_mut();
	if let Some(ref matches) = matches.subcommand_matches("spec") {
		// TODO: spec
		return;
	}

	if let Some(rootfs_v) = matches.value_of("rootfs") {
		rootfs = rootfs_v;
	}else{
		app.print_long_help();
		return;
	}

	if let Some(_cmd) = matches.values_of("command") {
		command_vec = matches.values_of("command").unwrap().collect();
	} else {
		app.print_long_help();
		return;
	}

	let c = command_vec[0];
	let (command, args) = determine_command_tuple(&c, &command_vec).ok().unwrap_or_else(|| {
		app.print_long_help();
		process::exit(7);
	});

	runtime::run_container(&rootfs, command, args);
}

/// Determines based on the inputs whether we are going to invoke a shell, with
/// shell interpretation, or a simple unescaped argument vector.
/// Rearranges arguments as needed, but doesn't reallocate.
fn determine_command_tuple<'a, T: AsRef<str> + 'a>(shell_command: &'a T, argv: &'a [T]) -> Result<(&'a str, Vec<&'a str>), Box<dyn Error>> {
	let mut vec: Vec<&str> = vec![];

	// Prepend shell and command string if a command string is given.
	//if let Some(shell_command) = shell_command {
	vec.push("/bin/sh");
	vec.push("-c");
	//vec.push(shell_command.as_ref());
	//}

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
