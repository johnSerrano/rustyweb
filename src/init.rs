use config::ConfigStruct;
use std::fs;


pub fn init(mut config: ConfigStruct) -> ConfigStruct {
    check_error_pages();

    // If index is empty, default to "index.html"
    // in the future, this should handle index as an optional parameter
    if config.index.len() == 0 {
        config.index = "/index.html".to_string();
    }

    // Make sure index starts with a / (if not, append)
    let first_char_of_index = config.index.as_str().chars().nth(0).unwrap();
    match first_char_of_index {
        '/' => {
            ;
        }
        _ => {
            let mut tmp = "/".to_string();
            tmp.push_str(config.index.as_str());
            config.index = tmp;
        }
    }

    // Make sure path_to_files is a directory
    if !fs::metadata(config.path_to_files.as_str()).unwrap().is_dir() {
        panic!("path_to_files is not a directory.");
    }

    return config;
}


// Verify all necessary error pages exist.
fn check_error_pages() {
    let expected_paths = &["/etc/rustyweb/errorpages/404.html",
                           "/etc/rustyweb/errorpages/generic.html"];

    for path in expected_paths {
        if !fs::metadata(path).is_ok() {
            panic!("Missing error page: {}", path);
        }
    }
}
