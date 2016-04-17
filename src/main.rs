extern crate rustc_serialize;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::fs::File;
use std::path::Path;
use std::str;
use rustc_serialize::json;


// Struct to contain values from the configuration file
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ConfigStruct {
    port: u32,
    path_to_files: String,
}

// struct to store data from a HTTP request
pub struct HTTPRequestStruct {
    request_type: String,
    location: String,
    protocol: String,
}


fn main() {
    let config = read_config_files();
    run_server(config);
}


// Runs server. Takes ConfigStruct
// (which most likely should come from read_config_files).
fn run_server(config: ConfigStruct) {
    let port = config.port;
    let address = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&*address).unwrap();
    println!("Listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let config_clone = config.clone();
                thread::spawn(|| {
                    handle_connection(s, config_clone);
                });
            }
            Err(err) => {
                println!("Error {}", err);
            }
        }
    }
}


// Handle the TCP connection
fn handle_connection(mut stream: TcpStream, config: ConfigStruct) {
    let mut buffer = [0; 4096];
    match stream.read(&mut buffer) {
        Ok(_) => {
            ;
        }
        Err(err) => {
            println!("Error reading stream. {}", err);
        }
    }
    let request = parse_request(buffer);
    if request.request_type == "GET" {
        serve_get(&request.location, stream, config);
    }
    println!("Type: {}", request.request_type);
    println!("Location: {}", request.location);
    println!("Protocol: {}", request.protocol);
}


// Serve up a file!
fn serve_get(location: &String, mut stream: TcpStream, config: ConfigStruct) {
    println!("serving GET request: {}{}", config.path_to_files, location);
    let file_address = format!("{}{}", config.path_to_files, location);
    let path = &Path::new(&*file_address);
    let mut file_to_serve = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            // TODO: serve 404
            println!("404");
            return;
        }
    };
    let mut byte_vector: Vec<u8> = Vec::new();
    match file_to_serve.read_to_end(&mut byte_vector) {
        Ok(_) => {
            ;
        }
        Err(err) => {
            // Serve 404
            println!("404");
            return;
        }
    }

    let mut buffer = [0; 4096];
    let iterations = (byte_vector.len() / 4096) + 1;
    println!("{}", iterations);

    for i in 0..iterations {
        for j in 0..4096 {
            if (byte_vector.len() <= j + (4096 * i)) {
                break;
            }
            buffer[j] = byte_vector[j + (4096 * i)]
        }
        stream.write(&buffer);
        println!("Iteration!");
    }

    //stream.write(bytes_to_serve);
    // TODO: serve file
}


// reads the incoming stream and extracts relevant information from it.
fn parse_request(buffer: [u8; 4096]) -> HTTPRequestStruct {
    let request = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let request_array = request.split("\n").collect::<Vec<&str>>();
    let request_type = request_array[0]
                           .split(" ")
                           .collect::<Vec<&str>>()[0];
    let request_location = request_array[0]
                               .split(" ")
                               .collect::<Vec<&str>>()[1];
    let request_protocol = request_array[0]
                               .split(" ")
                               .collect::<Vec<&str>>()[2];
    let request_struct = HTTPRequestStruct {
        request_type: request_type.to_string(),
        location: request_location.to_string(),
        protocol: request_protocol.to_string(),
    };
    return request_struct;
}


// Read config file. Returns ConfigStruct containing config file information.
fn read_config_files() -> ConfigStruct {
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
