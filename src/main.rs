extern crate rustc_serialize;

mod config;
mod utils;
mod init;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::str;
use config::read_config_files;
use config::ConfigStruct;
use utils::serve_data;
use utils::get_file_from_location;
use init::init;

// struct to store data from a HTTP request
pub struct HTTPRequestStruct {
    request_type: String,
    location: String,
    protocol: String,
}


fn main() {
    let config = read_config_files();
    let config = init(config);
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
    //println!("Type: {}", request.request_type);
    //println!("Location: {}", request.location);
    //println!("Protocol: {}", request.protocol);
}


// Serve up a file!
fn serve_get(location: &String,	 stream: TcpStream, config: ConfigStruct) {
    let mut file = get_file_from_location(location, config);

    let mut byte_vector: Vec<u8> = Vec::new();
    match file.read_to_end(&mut byte_vector) {
        Ok(_) => {
            ;
        }
        Err(_) => {
            println!("ERROR: Failed to read file as bytes. Aborting connection.");
            return;
        }
    }

    serve_data(byte_vector, stream);
}


// Reads the incoming stream and extracts relevant information from it.
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

// TESTS ***********************************************************************

#[test]
fn test_server_accepts_incoming_streams() {
    use std::process::Command;

    let config = config::parse_file("test_files/test_server_valid.json");
    let config_clone = config.clone();
    // run server on port 7999
    thread::spawn(|| {
        run_server(config_clone);
    });

    // exec curl
    let curl = Command::new("curl").arg("http://localhost:7999/index.html").output().unwrap();
    let mut output = format!("{}", String::from_utf8_lossy(&curl.stdout));
    while output.pop().unwrap() == '\u{0}' { ; }
    assert_eq!("Index Received", output);
}

#[test]
fn test_index_pages() {
    use std::process::Command;

    let config = config::parse_file("test_files/test_server_valid.json");
    let config_clone = config.clone();
    // run server on port 7999
    thread::spawn(|| {
        run_server(config_clone);
    });

    // exec curl
    let curl = Command::new("curl").arg("http://localhost:7999/").output().unwrap();
    let mut output = format!("{}", String::from_utf8_lossy(&curl.stdout));
    while output.pop().unwrap() == '\u{0}' { ; }
    assert_eq!("Index Received", output);

    let curl = Command::new("curl").arg("http://localhost:7999/dir").output().unwrap();
    let mut output = format!("{}", String::from_utf8_lossy(&curl.stdout));
    while output.pop().unwrap() == '\u{0}' { ; }
    assert_eq!("Index Received", output);
}

#[test]
fn test_404() {
    use std::process::Command;

    let config = config::parse_file("test_files/test_server_valid.json");
    let config_clone = config.clone();
    // run server on port 7999
    thread::spawn(|| {
        run_server(config_clone);
    });

    //TODO verify /not/a/file does not exist

    // exec curl
    let curl = Command::new("curl").arg("http://localhost:7999/not/a/file").output().unwrap();
    let mut output = format!("{}", String::from_utf8_lossy(&curl.stdout));
    while output.pop().unwrap() == '\u{0}' { ; }
    assert_eq!("404 Received", output);
}
