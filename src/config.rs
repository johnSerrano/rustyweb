use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ConfigStruct {
    pub port: u32,
    pub path_to_files: String,
    pub index: String,
}

// Read config file. Returns ConfigStruct containing config file information.
pub fn read_config_files() -> ConfigStruct {
    let path = &Path::new("/etc/rustyweb/rustyweb.conf");
    let mut config_file = match File::open(path) {
        Ok(f) => f,
        Err(err) => panic!("{}", err),
    };
    let mut contents = String::new();
    match config_file.read_to_string(&mut contents) {
        Ok(s) => s,
        Err(err) => panic!("{}", err),
    };
    let config: ConfigStruct = json::decode(&*contents).unwrap();
    return config;
}
