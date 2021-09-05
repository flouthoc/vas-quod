use nix::sched;
use nix::sys::signal::Signal;
use nix::unistd;
use std::process::{Command, ExitStatus};
use nix::sys::wait::{waitpid, WaitStatus, WaitPidFlag};

use crate::cgroup;
use crate::filesystem;
use crate::mount;
use crate::namespace;
use std::cell::RefCell;

static CGROUP_NAME: &str = "vasquod-container";
static HOSTNAME: &str = "vasquod";

fn set_hostname(hostname: &str){
	// can also use libc here
	unistd::sethostname(hostname).unwrap()
}

struct Runner<'l> {
	command: &'l str,
	command_args: &'l [&'l str]
}

impl<'l> Runner<'l> {
	fn run(self) -> isize {
		let exit_status: ExitStatus = Command::new(self.command).args(self.command_args)
			.spawn().expect("Failed to execute container command").wait().unwrap();
		match exit_status.code() {
			Some(code) => code as isize,
			None => -1
		}
	}
}

impl Drop for Runner<'_> {
	fn drop(&mut self) {
		mount::unmount_proc();
	}
}

fn spawn_child<'l, 's : 'l>(hostname: &str, cgroup_name: &str, rootfs: &str, command: &'s str, command_args: &'s [&'s str]) -> isize {

	namespace::create_isolated_namespace();
	cgroup::cgroup_init(cgroup_name);
	set_hostname(hostname);

	mount::mount_root_fs(rootfs);
	filesystem::set_root_fs(rootfs);
	mount::unmount_host_root_fs();
	mount::mount_proc();

	// The Drop impl for Runner is the equivalent of a try/finally
	// block to ensure we unmount regardless of what goes wrong
	let run : Runner<'l> = Runner { command, command_args };
	run.run()
}

pub fn run_container(rootfs: &str, command: &str, command_args: Vec<&str>) -> nix::Result<WaitStatus> {

	let group_name = CGROUP_NAME;
	let hostname = HOSTNAME;
	const STACK_SIZE: usize = 1024 * 1024;
	let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];

	let cb = Box::new(|| spawn_child(hostname, group_name, rootfs, command,
									 command_args.as_slice()));

	//See `man clone`
	let clone_flags =
		sched::CloneFlags::CLONE_NEWNS | sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWCGROUP | sched::CloneFlags::CLONE_NEWUTS | sched::CloneFlags::CLONE_NEWIPC | sched::CloneFlags::CLONE_NEWNET;
	let child_pid = sched::clone(cb, stack, clone_flags, Some(Signal::SIGCHLD as i32))
		.expect("Failed to create child process");
	waitpid(child_pid, None)
}
