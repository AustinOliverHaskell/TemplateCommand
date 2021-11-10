pub fn load_template_file(paths: &Vec<String>, template_file: &String, be_verbose: bool) -> Option<String> {
    
    for path in paths {
        let template_path: String = String::from(path) + template_file;

        if be_verbose {
            println!("Attempting to load file: {:}", &template_path);
        }

        let possible_template_file_data = std::fs::read_to_string(&template_path);
        if possible_template_file_data.is_err() {
            if be_verbose {
                println!("{:?} is no good - {:?}", &template_path, possible_template_file_data.unwrap_err());
            }
            continue;
        }

        let template_file_data = possible_template_file_data.unwrap();

        return Some(template_file_data);
    }

    None
}

pub fn create_files(path: String, file_names: &Vec<String>, file_data: &String) {

    

}