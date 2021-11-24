use std::fs::{ File, OpenOptions, read_to_string };
use std::io::Write;

use crate::platform_specific::PLATFORM_SEPARATOR_SLASH;

pub fn load_template_file(template_file_dir_path: &String, template_file_name: &String, be_verbose: bool) -> Option<String>{

    let template_path: String = String::from(template_file_dir_path) + PLATFORM_SEPARATOR_SLASH + template_file_name;

    if be_verbose {
        println!("Attempting to load template file: {:}", template_path);
    }

    let possible_template_file_data = read_to_string(&template_path);
    if possible_template_file_data.is_err() {
        println!("Failed to load template file. Reason: {:}", possible_template_file_data.unwrap_err());
        return None;
    }

    let template_file_data = possible_template_file_data.unwrap();

    Some(template_file_data)
}

pub fn write_file(path: &String, file_contents: &String, be_verbose: bool, overwrite: bool) {
    let mut possible_file = OpenOptions::new().write(true).open(path);

    if be_verbose {
        println!("Attempting to write file {:}", path);
    }

    if possible_file.is_ok() && overwrite == false {
        println!("Skipping file {:} since it already exists and -o isn't present. ", path);
        return;
    } else if possible_file.is_err() {
        possible_file = File::create(path);
        match possible_file {
            Err(e) => {
                println!("Unable to create file {:} reason: {:}", path, e); 
                return;
            }
            _ => {}
        }
    }

    let mut file = possible_file.unwrap();

    match file.write_all(file_contents.as_bytes()) {
        Err(e) => println!("Failed to write contents of file. Reason: {:}", e),
        _ => {
            if be_verbose {
                println!("Wrote File!"); 
            }
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