use crate::file_manip::*;
use crate::platform_specific::PLATFORM_SEPARATOR_SLASH;

use log::*;

const TEMPLATE_FILE_START: &str = "template.";

// This const will come into play when the shove header functionality is implemented. 
const HEADER_FILE_START: &str = "header.";

#[derive(Debug)]
pub struct TemplateFile {
    pub template_file_extension: String, 
    pub template_file_path: String, 
    pub template_file_data: String
}

impl TemplateFile {
    pub fn new(
        extension_list: &Vec<String>, 
        root_path: &String) 
    -> Option<Self> {

        TemplateFile::generic_new(extension_list, root_path, TEMPLATE_FILE_START)
    }

    pub fn new_header(extension_list: &Vec<String>, 
        root_path: &String) 
    -> Option<Self> {
        TemplateFile::generic_new(extension_list, root_path, HEADER_FILE_START)
    }

    fn generic_new(
        extension_list: &Vec<String>, 
        root_path: &String, 
        file_name_start: &str) 
    -> Option<Self> {

        let found_template_file_matching_extension = figure_out_which_template_to_use(extension_list, root_path, file_name_start);
        if found_template_file_matching_extension.is_none() {
            error!("Failed to find template file with any combo of extensions {:}", extension_list.join("."));
            return None;
        }

        let template_file_extension    = found_template_file_matching_extension.unwrap();
        let template_file_path: String = file_name_start.to_string() + &template_file_extension;
        let template_file_data         = load_file(root_path, &template_file_path);
        if template_file_data.is_none() {
            error!("Failed to load template file {:}", template_file_path);
            return None;
        }

        Some(Self {
            template_file_extension: template_file_extension.clone(),
            template_file_path: root_path.clone() + file_name_start + &template_file_extension,
            template_file_data: template_file_data.unwrap()
        })
    }
}

pub fn figure_out_which_template_to_use(extension_list: &Vec<String>, root_path: &String, file_name_start: &str) -> Option<String> {

    let mut extension_list_copy = extension_list.clone();

    let mut file_name = String::from(root_path) + PLATFORM_SEPARATOR_SLASH + file_name_start + &extension_list.join(".");

    info!("Checking to see if {:} exists", file_name);
    
    while !check_if_file_exists(&file_name) {
        if extension_list_copy.len() == 1 {
            // No file exists, we just failed the file exists check. 
            return None;
        }
    
        extension_list_copy.remove(0);
        file_name = String::from(root_path) + PLATFORM_SEPARATOR_SLASH + file_name_start + &extension_list_copy.join(".");
    
        info!("Checking to see if {:} exists", file_name);
    }
    
    Some(extension_list_copy.join("."))
}

pub fn does_template_contain_forced_filename(template_data: &str) -> Option<String>{

    use crate::parser::*;
    use crate::token::*;

    // If force_file_name is not the first token, we ignore it. 
    let first_token = Parser::find_first_token(template_data);

    if first_token.is_none() { return None; }
    let first_token = first_token.unwrap();

    let token = Token::from_string(&template_data[first_token.start..first_token.end]);

    if token.is_err() { return None; }
    let token = token.unwrap();

    if token.id != "FORCE_FILE_NAME".to_string() { return None; }

    if token.has_variables() == false { error!("FORCE_FILE_NAME was provided but contains no file name. "); return None; }

    let file_name = &token.variables.unwrap()[0].variable_list[0];

    Some(file_name.to_string())
}