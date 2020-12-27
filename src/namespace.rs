use nix::sched;

pub fn create_isolated_namespace(){
	sched::unshare(sched::CloneFlags::CLONE_NEWNS).expect("Failed to unshare");
}
