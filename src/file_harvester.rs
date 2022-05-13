use crate::file_manip::*;
use crate::platform_specific::*;
use crate::util::*;

use log::*;

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

        //println!("File Name {{{:}}}  extension {{{:}}}", &file_name, &extension);
        HarvestedFile {
            extension: Some(extension),
            file_name: Some(file_name),
            path: Some(String::from(path))
        }
    }
}

// @todo: finish this, it should probs return a type with the extension, file name, and path contained within. 
pub fn harvest_files_from_dir(dir: &Option<String>, include_list: &Vec<String>) 
    -> Vec<HarvestedFile> {

    let raw_file_list = harvest_all_files_in_dir(dir, include_list);
    if raw_file_list.is_none() {
        return Vec::new();
    }

    let mut harvested_file_list: Vec<HarvestedFile> = Vec::new();
    for raw_file in &raw_file_list.unwrap() {
        harvested_file_list.push(HarvestedFile::from_string(raw_file));
    }

    harvested_file_list
}

pub fn harvest_files_from_dir_as_string(dir: &Option<String>, include_list: &Vec<String>, write_file_names_with_path: bool) -> String {
    
    let harvested_files = harvest_files_from_dir(dir, include_list);

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

pub fn harvest_all_files_in_dir(directory_path: &Option<String>, include_list: &Vec<String>) -> Option<Vec<String>> {

    // @Refactor: Same comment as below on like 108
    let include_all_files: bool = include_list.is_empty();

    let pwd: String;
    if directory_path.is_none() {
        // @Refactor: This could be a level higher, kinda weird having this non-obvious behavior as part of the function. - Austin Haskell
        let temp_pwd = get_current_path();
        if temp_pwd.is_none() {
            error!("Failed to figure out which directory to harvest. Was unable to get current path. ");
            return None;
        }
        pwd = temp_pwd.unwrap();
    } else {
        pwd = directory_path.clone().unwrap();
    }

    let files = std::fs::read_dir(pwd);
    if files.is_err() {
        error!("Unable to read directory information");
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

            info!("Found file {:}", &file_name);

            if include_list.contains(&file_name) {
                let explicitly_included_file = file_path.clone().into_os_string().into_string().unwrap();
                file_list.push(explicitly_included_file);
                continue;
            }

            let file_extension   = extract_extension_from_file_name(&file_name);
            if file_extension.is_some() && !include_list.contains(&file_extension.clone().unwrap()) && !include_all_files{
                // Not including all files and this extension isnt on the include list
                info!("Ignoring file {:} because it's extension is not on the include list. ", file_name);
                continue;
            }

            if file_extension.is_none() {
                info!("Skipping file {:} because it was not explicitly included and has no extension. ", file_name);
                continue;
            }

            info!("File extension for file {:} is {:?}", file_name, file_extension);
            
            let file_full_path_and_name = file_path.clone().into_os_string().into_string();
            if file_full_path_and_name.is_err() {
                continue;
            }
            file_list.push(file_full_path_and_name.unwrap());
        }
    }

    for file in &file_list {
        info!("Found file for harvest: {:?}", file);
    }

    Some(file_list)
}

// @todo: Harvest all subdirs command
/*pub fn find_all_subdirs(root_path: &str) -> Vec<String> {
    let files = std::fs::read_dir(root_path);
    if files.is_err() {
        error!("Unable to read directory information");
    }

    let subdir_list: Vec<String> = Vec::new();
    for file_result in files.unwrap() {
        if file_result.is_err() {
            continue;
        }

        let file             = &file_result.unwrap();
        let file_description = file.file_type().unwrap();

        if !file_description.is_file() {
            info!("Found dir: {:?}", file);
        }
    }

    subdir_list
}*/