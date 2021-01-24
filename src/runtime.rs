use nix::sched;
use nix::sys::signal::Signal;
use nix::unistd;
use std::process::Command;

use crate::cgroup;
use crate::filesystem;
use crate::mount;
use crate::namespace;

static CGROUP_NAME: &str = "vasquod-container";
static HOSTNAME: &str = "vasquod";

fn set_hostname(hostname: &str){
	// can also use libc here
	unistd::sethostname(hostname).unwrap()
}

fn spawn_child(hostname: &str, cgroup_name: &str, rootfs: &str, command: &str, command_args: &[&str]) -> isize {

	namespace::create_isolated_namespace();	
	cgroup::cgroup_init(cgroup_name);
	set_hostname(hostname);

	mount::mount_root_fs(rootfs);	
	filesystem::set_root_fs(rootfs);
	mount::unmount_host_root_fs();
	mount::mount_proc();	
	
	Command::new(command).args(command_args).spawn().expect("Failed to execute container command").wait().unwrap();

	mount::unmount_proc();
	return 0;
}

pub fn run_container(rootfs: &str, command: &str, command_args: Vec<&str>){

	let group_name = CGROUP_NAME;
	let hostname = HOSTNAME;
	const STACK_SIZE: usize = 1024 * 1024;
	let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];
	
	let cb = Box::new(|| spawn_child(hostname, group_name, rootfs, command, command_args.as_slice()));

	//See `man clone`
	let clone_flags = sched::CloneFlags::CLONE_NEWNS | sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWCGROUP | sched::CloneFlags::CLONE_NEWUTS | sched::CloneFlags::CLONE_NEWIPC | sched::CloneFlags::CLONE_NEWNET;
	let _child_pid = sched::clone(cb, stack, clone_flags, Some(Signal::SIGCHLD as i32)).expect("Failed to create child process");

}
