// Make sure error pages exist
// Make sure index starts with a / (if not, append)
// Make sure path_to_files is a directory (else panic)

use config::ConfigStruct;
use std::path::Path;
use std::fs;


pub fn init(config: ConfigStruct) -> ConfigStruct {
    check_error_pages();

    // make sure index starts with a / or add it
    // make sure path_to_files is a directory
    return config;
}


// Verify all necessary error pages exist. Crash otherwise.
fn check_error_pages() {
    let expected_paths = &["/etc/rustyweb/errorpages/404.html",
                           "/etc/rustyweb/errorpages/generic.html"];

    for path in expected_paths {
        if !fs::metadata(path).is_ok() {
            panic!("Missing error page: {}", path);
        }
    }
}
