use std::fs::{ File, OpenOptions, read_to_string };
use std::io::Write;

pub fn load_template_file_from_multiple_search_paths(paths: &Vec<String>, template_file_name: &String, be_verbose: bool) -> Option<String> {
    
    for path in paths {

        if be_verbose {
            println!("Attempting to load file: {:}{:}", path, &template_file_name);
        }

        let template = load_template_file(path, template_file_name, be_verbose);
        if template.is_none() {
            if be_verbose {
                println!("Failed to load template file: {:}{:}", &path, template_file_name);
            }
            continue;
        }

        return Some(template.unwrap());
    }

    None
}

pub fn load_template_file(template_file_dir_path: &String, template_file_name: &String, be_verbose: bool) -> Option<String>{

    let template_path: String = String::from(template_file_dir_path) + template_file_name;

    let possible_template_file_data = read_to_string(&template_path);
    if possible_template_file_data.is_err() {
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