use std::fs;
use std::path::PathBuf;
use nix::unistd;
use nix::sys::stat;
use nix::Error;

static ROOT_PATH: &str = "/";
static OLD_ROOT_PATH: &str = ".oldroot";

pub fn set_root_fs(rootfs: &str){
	
	let p_root_fs = PathBuf::from(rootfs).join(OLD_ROOT_PATH);
	let _rm_status = fs::remove_dir_all(&p_root_fs).map_err(|_| Error::InvalidPath);
	let _mkdir_status = unistd::mkdir(&p_root_fs, stat::Mode::S_IRWXU | stat::Mode::S_IRWXG | stat::Mode::S_IRWXO,);
	let _pivot_root_status = unistd::pivot_root(rootfs, &p_root_fs);
	let _chdir_status = unistd::chdir(ROOT_PATH);
}
