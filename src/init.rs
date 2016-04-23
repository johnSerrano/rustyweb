use config::ConfigStruct;
use std::fs;


// Test environment and configuration to guarantee safe running.
// Any preventable or predictable runtime errors should be found here.
pub fn init(mut config: ConfigStruct) -> ConfigStruct {
    check_error_pages(&config);

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

    //TODO: check path to error pages

    return config;
}


// Verify all necessary error pages exist.
fn check_error_pages(config: &ConfigStruct) {
    let expected_paths = &[format!("{}/404.html", config.path_to_error_pages),
                           format!("{}/generic.html", config.path_to_error_pages)];

    for path in expected_paths {
        if !fs::metadata(path).is_ok() {
            panic!("Missing error page: {}", path);
        }
    }
}


// TESTS ***********************************************************************

#[test]
fn test_empty_index() {
    use config::parse_file;
    let result = init(parse_file("./test_files/config_empty_index.json"));
    assert!(result.index == "/index.html");
}

#[test]
fn test_no_slash_before_index() {
    use config::parse_file;
    let result = init(parse_file("./test_files/config_index_no_slash.json"));
    assert!(result.index == "/index.html");
}

#[test]
#[should_panic]
fn test_path_to_files_not_dir() {
    use config::parse_file;
    init(parse_file("./test_files/config_path_to_files_not_dir.json"));
}

#[test]
#[should_panic]
fn test_path_to_files_does_not_exist() {
    use config::parse_file;
    init(parse_file("./test_files/config_path_to_files_not_exist.json"));
}

#[test]
#[should_panic]
fn test_path_to_files_does_not_empty() {
    use config::parse_file;
    init(parse_file("./test_files/config_path_to_files_empty.json"));
}

#[test]
#[should_panic]
fn test_port_empty() {
    use config::parse_file;
    init(parse_file("./test_files/config_port_empty.json"));
}

#[test]
#[should_panic]
fn test_port_out_of_range() {
    use config::parse_file;
    init(parse_file("./test_files/config_port_out_of_range.json"));
}

#[test]
#[should_panic]
fn test_port_negative() {
    use config::parse_file;
    init(parse_file("./test_files/config_port_negative.json"));
}

#[test]
#[should_panic]
fn test_errorpages_missing() {
    use config::parse_file;
    init(parse_file("./test_files/config_errorpages_missing.json"));
}

#[test]
#[should_panic]
fn test_errorpages_not_dir() {
    use config::parse_file;
    init(parse_file("./test_files/config_errorpages_not_dir.json"));
}
