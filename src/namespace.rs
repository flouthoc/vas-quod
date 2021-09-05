use nix::sched;

pub fn create_isolated_namespace(){
	// Unshare mount, network, IPC and UTS namespace
	sched::unshare(sched::CloneFlags::CLONE_NEWNS | sched::CloneFlags::CLONE_NEWNET | sched::CloneFlags::CLONE_NEWUTS
	| sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWUTS).expect("Failed to unshare");
}
