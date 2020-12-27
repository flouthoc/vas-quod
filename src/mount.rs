use nix::mount;

static PROC: &str = "proc";

pub fn mount_proc(){
	const NONE: Option<&'static [u8]> = None;
	mount::mount(Some(PROC), PROC, Some(PROC), mount::MsFlags::empty(), NONE).expect("Failed to mount the /proc");
}

pub fn unmount_proc(){
	mount::umount("proc").unwrap();
}
	
