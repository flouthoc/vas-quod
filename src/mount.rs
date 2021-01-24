use nix::mount;

static PROC: &str = "proc";
static OLD_ROOT_PATH: &str = "/.oldroot";

pub fn mount_proc(){
	const NONE: Option<&'static [u8]> = None;
	mount::mount(Some(PROC), PROC, Some(PROC), mount::MsFlags::empty(), NONE).expect("Failed to mount the /proc");
}

pub fn mount_root_fs(rootfs: &str){
	let _status = mount::mount(Some(rootfs),rootfs,None::<&str>,mount::MsFlags::MS_BIND | mount::MsFlags::MS_REC,None::<&str>,);
}

pub fn unmount_host_root_fs(){
	let _status = mount::umount2(OLD_ROOT_PATH, mount::MntFlags::MNT_DETACH);
}

pub fn unmount_proc(){
	mount::umount("proc").unwrap();
}
	
