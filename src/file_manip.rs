use std::fs::{ File, OpenOptions, read_to_string };
use std::io::Write;

use log::*;

use crate::platform_specific::PLATFORM_SEPARATOR_SLASH;

pub fn load_file(file_dir: &String, file_name: &String) -> Option<String>{

    let file_path = if *file_dir == "" { file_name.to_string() } else { String::from(file_dir) + PLATFORM_SEPARATOR_SLASH + file_name };

    info!("Attempting to load file: {:}", file_path);
    
    let possible_file_data = read_to_string(&file_path);
    if possible_file_data.is_err() {
        println!("Failed to load file. Reason: {:}", possible_file_data.unwrap_err());
        return None;
    }

    let file_data = possible_file_data.unwrap();

    Some(file_data)
}

pub fn write_file(path: &String, file_contents: &String, overwrite: bool) {
    let mut possible_file = OpenOptions::new().write(true).open(path);

    info!("Attempting to write file {:}", path);

    // File is okay means that we were able to open a file that already exists
    if possible_file.is_ok() && overwrite == false {
        error!("Skipping file {:} since it already exists and -o isn't present. ", path);
        return;
    } else if possible_file.is_ok() && overwrite {
        // Open again with truncate on. 
        possible_file = OpenOptions::new().write(true).truncate(true).open(path);
    } else if possible_file.is_err() {
        possible_file = File::create(path);
        match possible_file {
            Err(e) => {
                error!("Unable to create file {:} reason: {:}", path, e); 
                return;
            }
            _ => {}
        }
    }

    let mut file = possible_file.unwrap();

    match file.write_all(file_contents.as_bytes()) {
        Err(e) => error!("Failed to write contents of file. Reason: {:}", e),
        _ => {
            info!("Wrote File!"); 
        }
    }
}

pub fn check_if_file_exists(file: &String) -> bool {
    let possible_template_file_data = read_to_string(&file);
    if possible_template_file_data.is_err() {
        return false;
    } 

    true
}

pub fn get_current_dir_name() -> Option<String>{
    let path = std::env::current_dir();

    if path.is_err() {
        return None;
    }

    let full_path = path.unwrap();
    let mut prefix    = full_path.clone();
    prefix.pop();

    let dir_name = full_path.strip_prefix(prefix);

    if dir_name.is_err() {
        None
    } else {
        // Holy crap this line sucks. - Austin Haskell 
        Some(dir_name.unwrap().as_os_str().to_os_string().into_string().unwrap_or(String::from("")))
    }
}

pub fn get_current_path() -> Option<String> {
    let path = std::env::current_dir();

    if path.is_err() {
        return None;
    }

    let string_path = path.unwrap().into_os_string().into_string();

    if string_path.is_err() {
        println!("Failed to convert os string into regular string. ");
        return None;
    }

    Some(string_path.unwrap())
}