use crate::file_manip::*;
use crate::platform_specific::*;

pub struct HarvestedFile {
    pub extension: String,
    pub file_name: String,
    pub path: String
}

impl HarvestedFile {
    pub fn to_string(self: &Self) -> String {
        return self.file_name.clone() + &self.extension;
    }
}

pub fn harvest_files_from_dir_as_string(dir: &Option<String>, ignore_list: Vec<String>, be_verbose: bool) -> String {

    let remove_path: bool = dir.is_none();

    let file_list = get_all_files_in_dir(dir, ignore_list, be_verbose);
    if file_list.is_none() {
        return String::new();
    }

    let mut dir_list_on_separate_lines: String = String::new();
    for file in file_list.unwrap() {

        let mut file_name: String = file.clone();
        if remove_path {
            let stripped_file_name = extract_file_name_and_extension_from_path(&file);
            if stripped_file_name.is_none() {
                continue;
            }
            file_name = stripped_file_name.unwrap();
        }
        dir_list_on_separate_lines += &(file_name + PLATFORM_LINE_ENDING);
    }

    dir_list_on_separate_lines
}

// @todo: finish this, it should probs return a type with the extension, file name, and path contained within. 
pub fn harvest_files_from_dir(dir: &Option<String>, ignore_list: Vec<String>, be_verbose: bool) 
    -> Vec<HarvestedFile> {


    Vec::new()
}