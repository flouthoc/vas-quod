extern crate libocispec;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use nix::unistd;
use nix::sys::stat;
use nix::Error;

use crate::runtime;

// TODO: Add RunOpts and use that instead of individual args

pub fn run(config: &str, bundle: &str) {
	let mut base_config: &str = "config.json";
	if(!config.is_empty()){
		base_config = config;
	}
	
	let spec: libocispec::runtime::Spec = match libocispec::runtime::Spec::load(base_config) {
		Ok(spec) => spec,
		Err(e) => panic!("{}", e),
	};
	
	// TODO: Implement core	
	runtime::run_container(&spec.root.as_ref().unwrap().path, 
				spec.process.as_ref().unwrap().args.as_ref().unwrap().clone().into_iter().nth(0).as_ref().unwrap(), 
				[].to_vec());
}
