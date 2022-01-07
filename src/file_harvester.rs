use crate::file_manip::*;
use crate::platform_specific::*;
use crate::util::*;

#[derive(Debug)]
pub struct HarvestedFile {
    pub extension: Option<String>,
    pub file_name: Option<String>,
    pub path:      Option<String>
}

impl HarvestedFile {
    pub fn to_string(self: &Self) -> String {
        return 
            replace_if_not_none("", &self.file_name) +
            "." +  
            &replace_if_not_none("", &self.extension);
    }

    pub fn to_fully_expanded_string(self: &Self) -> String {
        return
            replace_if_not_none("", &self.path) + 
            &replace_if_not_none("", &self.file_name) + 
            &replace_if_not_none("", &self.extension);
    }

    pub fn from_string(path: &str) -> Self {
        let file_name_and_extension = extract_file_name_and_extension_from_path(path);
        if file_name_and_extension.is_none() {
            return HarvestedFile {
                extension: None,
                file_name: None,
                path: Some(String::from(path))
            }
        }
        let unwrapped_file_name_and_extension = file_name_and_extension.unwrap();

        let possible_extension = extract_extension_from_file_name(&unwrapped_file_name_and_extension);
        let possible_file_name = remove_extensions_from_file_name(&unwrapped_file_name_and_extension);

        let extension: String;
        let file_name: String;

        if possible_extension == None {
            extension = String::new();
        } else {
            extension = possible_extension.unwrap();
        }

        if possible_file_name == None {
            file_name = String::new();
        } else {
            file_name = possible_file_name.unwrap();
        }

        println!("File Name {{{:}}}  extension {{{:}}}", &file_name, &extension);
        HarvestedFile {
            extension: Some(extension),
            file_name: Some(file_name),
            path: Some(String::from(path))
        }
    }
}

// @todo: finish this, it should probs return a type with the extension, file name, and path contained within. 
pub fn harvest_files_from_dir(dir: &Option<String>, ignore_list: &Vec<String>, be_verbose: bool) 
    -> Vec<HarvestedFile> {

    let raw_file_list = harvest_all_files_in_dir(dir, ignore_list, be_verbose);
    if raw_file_list.is_none() {
        return Vec::new();
    }

    let mut harvested_file_list: Vec<HarvestedFile> = Vec::new();
    for raw_file in &raw_file_list.unwrap() {
        harvested_file_list.push(HarvestedFile::from_string(raw_file));
    }

    harvested_file_list
}

pub fn harvest_files_from_dir_as_string(dir: &Option<String>, ignore_list: &Vec<String>, be_verbose: bool, write_file_names_with_path: bool) -> String {
    
    let harvested_files = harvest_files_from_dir(dir, ignore_list, be_verbose);

    let mut return_string = String::new();
    for file in harvested_files {
        if write_file_names_with_path {
            return_string += &file.to_fully_expanded_string();
        } else {
            return_string += &file.to_string();
        }
        return_string += PLATFORM_LINE_ENDING; 
    }
    
    return_string
}

pub fn harvest_all_files_in_dir(directory_path: &Option<String>, ignore_list: &Vec<String>, be_verbose: bool) -> Option<Vec<String>> {

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

            if be_verbose {
                println!("Found file {:}", &file_name);
            }

            if ignore_list.contains(&file_name) {
                continue;
            }

            let file_extension   = extract_extension_from_file_name(&file_name);
            if file_extension.is_some() && ignore_list.contains(&file_extension.clone().unwrap()) {
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