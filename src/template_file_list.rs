use crate::file_manip::*;
use crate::platform_specific::PLATFORM_SEPARATOR_SLASH;

use log::*;

const TEMPLATE_FILE_START: &str = "template.";

// This const will come into play when the shove header functionality is implemented. 
#[allow(dead_code)]
const HEADER_FILE_START: &str = "header.";

#[derive(Debug)]
pub struct UnprocessedTemplateFile {
    pub template_file_extension: String, 
    pub template_file_path: String, 
    pub template_file_data: String
}

impl UnprocessedTemplateFile {
    pub fn new(
        extension_list: &Vec<String>, 
        root_path: &String) 
    -> Option<Self> {

        let found_template_file_matching_extension = figure_out_which_template_to_use(extension_list, root_path);
        if found_template_file_matching_extension.is_none() {
            error!("Failed to find template file with any combo of extensions {:}", extension_list.join("."));
            return None;
        }

        let template_file_extension    = found_template_file_matching_extension.unwrap();
        let template_file_path: String = String::from(TEMPLATE_FILE_START) + &template_file_extension;
        let template_file_data         = load_template_file(root_path, &template_file_path);
        if template_file_data.is_none() {
            error!("Failed to load template file {:}", template_file_path);
            return None;
        }

        Some(UnprocessedTemplateFile {
            template_file_extension: template_file_extension.clone(),
            template_file_path: root_path.clone() + TEMPLATE_FILE_START + &template_file_extension,
            template_file_data: template_file_data.unwrap()
        })
    }
}

pub fn figure_out_which_template_to_use(extension_list: &Vec<String>, root_path: &String) -> Option<String> {

    let mut extension_list_copy = extension_list.clone();

    let mut file_name = String::from(root_path) + PLATFORM_SEPARATOR_SLASH + &String::from(TEMPLATE_FILE_START) + &extension_list.join(".");

    info!("Checking to see if {:} exists", file_name);
    
    while !check_if_file_exists(&file_name) {
        if extension_list_copy.len() == 1 {
            // No file exists, we just failed the file exists check. 
            return None;
        }
    
        extension_list_copy.remove(0);
        file_name = String::from(root_path) + PLATFORM_SEPARATOR_SLASH + &String::from(TEMPLATE_FILE_START) + &extension_list_copy.join(".");
    
        info!("Checking to see if {:} exists", file_name);
    }
    
    Some(extension_list_copy.join("."))
}

