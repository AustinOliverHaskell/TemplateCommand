use std::fs::{ File, OpenOptions, read_to_string };
use std::io::Write;

use regex::*;

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

pub fn extract_extension_from_file_name(file: &str) -> Option<String>{

    let regex = Regex::new(r"\.(.*)").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(file).collect();

    if capture_groups.is_empty() {
        return None;
    }

    let capture = &capture_groups[0];
    let extension = capture.get(1).map_or("", |e| e.as_str());

    Some(String::from(extension))
}

pub fn extract_file_name_and_extension_from_path(file: &str) -> Option<String> {

    #[cfg(unix)]
    let regex = Regex::new(r"(/*.*/)*(.*$)").unwrap();

    #[cfg(windows)]
    let regex = Regex::new(r"(\\*.*\\)*(.*$)").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(file).collect();

    if capture_groups.is_empty() { 
        return None;
    }

    let capture = &capture_groups[0];

    let ending = capture.get(2).map_or("", |e| e.as_str());
    if ending == "" {
        None
    } else {
        Some(String::from(ending))
    }
} 

pub fn get_all_files_in_dir(directory_path: &Option<String>, ignore_list: Vec<String>, be_verbose: bool) -> Option<Vec<String>> {

    let pwd: String;
    if directory_path.is_none() {
        let temp_pwd = get_current_path();
        if temp_pwd.is_none() {
            println!("Failed to figure out which directory to harvest. Was unable to get current path. ");
            return None;
        }
        pwd = temp_pwd.unwrap();
    } else {
        pwd = directory_path.clone().unwrap();
    }

    let files = std::fs::read_dir(pwd);
    if files.is_err() {
        println!("Error while reading directory information");
    }

    let mut file_list: Vec<String> = Vec::new();
    for file_result in files.unwrap() {
        if file_result.is_err() {
            continue;
        }

        let file             = &file_result.unwrap();
        let file_description = file.file_type().unwrap();
        let file_path        = file.path();
        let file_name        = file.file_name().into_string().unwrap_or(String::from(""));

        if file_description.is_file() {
            if ignore_list.contains(&file_name) {
                continue;
            }

            let file_extension   = extract_extension_from_file_name(&file_name);
            if file_extension.is_some() && ignore_list.contains(&file_extension.unwrap()) {
                continue;
            }
            
            let file_full_path_and_name = file_path.clone().into_os_string().into_string();
            if file_full_path_and_name.is_err() {
                continue;
            }
            file_list.push(file_full_path_and_name.unwrap());
        }
    }

    if be_verbose {
        for file in &file_list {
            println!("Found file for harvest: {:?}", file);
        }
    }

    Some(file_list)
}

#[test]
fn extract_name_and_extension() {

    assert_eq!(extract_file_name_and_extension_from_path("/home/austin/test/"), None);

    assert_eq!(extract_file_name_and_extension_from_path("/home/austin/test/foo.txt").unwrap(), "foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path("home/foo.txt").unwrap(), "foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path("foo.txt").unwrap(), "foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path("/home/foo.txt").unwrap(), "foo.txt"); 
    assert_eq!(extract_file_name_and_extension_from_path("/home/foo").unwrap(), "foo"); 
}