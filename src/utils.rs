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


// Find file to serve for a given location. Returns the file that
// should be served.
pub fn get_file_from_location(location: &String, config: ConfigStruct) -> File {
    let file_address = format!("{}{}", config.path_to_files, location);
    let path = &Path::new(&*file_address);

    //TODO: return status code somehow
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("File not found, 404");
            // Existence of 404 page should be checked in init module.
            // It is correct to fail fatally here on its nonexistence.
            File::open(&(Path::new(&*format!("{}/404.html", config.path_to_error_pages)))).unwrap()
        }
    };

    // If location is a directory, serve index page
    let data = file.metadata().unwrap().file_type();

    if data.is_dir() {
        let file_address = format!("{}{}{}", config.path_to_files, location, config.index);
        file = match File::open(Path::new(&*file_address)) {
            Ok(f) => f,
            Err(_) => {
                println!("Index not found, 404");
                // Existence of 404 page should be checked in init module.
                // It is correct to fail fatally here on its nonexistence.
                File::open(&(Path::new(&*format!("{}/404.html", config.path_to_error_pages)))).unwrap()
            }
        };
    }
    return file;
}
