//mod config;

use std::net::TcpStream;
use std::io::prelude::*;
use config::ConfigStruct;
use std::path::Path;
use std::fs::File;

// Send byte data to stream
pub fn serve_data(byte_vector: Vec<u8>, mut stream: TcpStream) {

    let mut buffer = [0; 4096];
    let iterations = (byte_vector.len() / 4096) + 1;

    for i in 0..iterations {
        for j in 0..4096 {
            if byte_vector.len() <= j + (4096 * i) {
                break;
            }
            buffer[j] = byte_vector[j + (4096 * i)]
        }
        match stream.write(&buffer) {
            Ok(_) => {
                    ;
            }
            Err(err) => {
                println!("ERROR SERVING DATA: {}", err);
            }
        }
    }
}


// Find file to serve for a given location. Returns a path to the file that
// should be served.
pub fn get_file_from_location(location: &String, config: ConfigStruct) -> File {
    let file_address = format!("{}{}", config.path_to_files, location);
    let path = &Path::new(&*file_address);

    //TODO: Cause status code to be 404
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("File not found, 404");
            // Currently fails fatally on error opening 404 page.
            // Consider returning a result<&Path> or similar from this function.
            // Somewhat related, should error page location be in config?
            // If so, we could verify the necessary pages exist in config module
            // Otherwise we could still check for error pages on load
            // NOTE TO JOHN: WRITE INIT PROGRAM TO CHECK EVERYTHING IS IN ORDER
            File::open(&(Path::new("/etc/rustyweb/errorpages/404.html"))).unwrap()
        }
    };

    let data = file.metadata().unwrap().file_type();

    // TODO
    // If file is a directory: open index
    if data.is_dir() {
        let file_address = format!("{}{}{}", config.path_to_files, location, config.index);
        file = match File::open(Path::new(&*file_address)) {
            Ok(f) => f,
            Err(_) => {
                println!("Index not found, 404");
                // Existence of 404 page should be checked in init module.
                // It is correct to fail fatally here on its nonexistence.
                File::open(&(Path::new("/etc/rustyweb/errorpages/404.html"))).unwrap()
            }
        };
    }

    // If file does not exist, path = error file

    return file;
}
