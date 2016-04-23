use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ConfigStruct {
    pub port: u16,
    pub path_to_files: String,
    pub index: String,
    pub path_to_error_pages: String,
}

// Read config file. Returns ConfigStruct containing config file information.
pub fn read_config_files() -> ConfigStruct {
    return parse_file("/etc/rustyweb/rustyweb.conf");
}

pub fn parse_file(file_path: &str) -> ConfigStruct {
    let path = &Path::new(file_path);
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


// TESTS


#[test]
fn test_valid_config() {
    read_config_files();
}
