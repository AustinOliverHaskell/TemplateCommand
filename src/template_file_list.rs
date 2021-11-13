use crate::file_manip::*;

pub struct UnprocessedOutputFile {
    pub file_name: String,
    pub extension: String,
    pub template_file_path: String, 
    pub template_file_data: String
}

impl UnprocessedOutputFile {
    pub fn new(
        extension_list: &Vec<String>, 
        root_path: &String, 
        output_file_name: &String, 
        output_extension: &String, 
        be_verbose: bool) 
    -> Option<Self> {


        let found_template_file_matching_extension = figure_out_which_template_to_use(extension_list, root_path, be_verbose);
        if found_template_file_matching_extension.is_none() {
            println!("Failed to find template file with any combo of extensions {:}", extension_list.join("."));
            return None;
        }

        let template_file_extension    = found_template_file_matching_extension.unwrap();
        let template_file_path: String = String::from("tt.") + &template_file_extension;
        let template_file_data         = load_template_file(root_path, &template_file_path, be_verbose);
        if template_file_data.is_none() {
            println!("Failed to load template file {:}");
        }

        Some(UnprocessedOutputFile {
            file_name: output_file_name.clone(),
            extension: output_extension.clone(),

            template_file_path: root_path.clone() + "tt." + &template_file_extension,
            template_file_data: template_file_data.unwrap()
        })
    }
}

pub fn figure_out_which_template_to_use(extension_list: &Vec<String>, root_path: &String, be_verbose: bool) -> Option<String> {

    let mut extension_list_copy = extension_list.clone();
    let mut file_name = String::from(root_path) + &String::from("/tt.") + &extension_list.join(".");

    if be_verbose {
        println!("Checking to see if {:} exists", file_name);
    }
    
    while !check_if_file_exists(&file_name) {
        if extension_list_copy.len() == 1 {
            // No file exists, we just failed the file exists check. 
            return None;
        }
    
        extension_list_copy.remove(0);
        file_name = String::from(root_path) + &String::from("/tt.") + &extension_list_copy.join(".");
    
        println!("Checking to see if {:} exists", file_name);
    }
    
    Some(extension_list_copy.join("."))
}

pub fn create_matching_file(original: &String, extension: &String) -> Option<String> {
    None
}




