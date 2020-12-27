use nix::unistd;

static ROOT_PATH: &str = "/";

pub fn set_root_fs(rootfs: &str){
	unistd::chroot(rootfs).unwrap();
	let _status = unistd::chdir(ROOT_PATH);
}
