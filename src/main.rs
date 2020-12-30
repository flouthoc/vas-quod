extern crate getopts;
use getopts::Options;
use std::env;
use std::process;

mod runtime;
mod cgroup;
mod filesystem;
mod mount;
mod namespace;

fn run(rootfs: &str, command_string: &str) {
	let child_command_buffer = command_string.split(" ");
	let mut child_command_vector = child_command_buffer.collect::<Vec<&str>>();
	let command = child_command_vector[0];
	child_command_vector.drain(0..1);

	runtime::run_container(&rootfs, command, child_command_vector);
}

fn print_usage(program: &str, opts: &Options) {
	let brief = format!("Usage: {} vas-quod [options]", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let mut opts = Options::new();
	opts.optopt("r", "rootfs", "Path to root file-system eg. --rootfs /home/alpinefs", "path");
	opts.optopt("c", "command", "Command to be executed eg. --command `curl http://google.com`", "command");
	opts.optflag("h", "help", "print this help menu");

	let matches = opts.parse(&args[1..]).ok().unwrap_or_else(|| {
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
		println!("Error: Please pass: --rootfs");
		print_usage(&program, &opts);
		process::exit(7);
	});
	let command_string = matches.opt_str("c").unwrap_or_else(|| {
		println!("Error: Please pass: --command");
		print_usage(&program, &opts);
		process::exit(7);
	});

	run(&rootfs, &command_string);
}
