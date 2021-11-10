use regex::*;

pub fn replace_symbols(template: &String, file_name: &String) -> String {
    let regex = Regex::new("\\[\\][A-Z_]+\\[\\]").unwrap();

    let possible_matches = regex.find(&template);
    if possible_matches.is_none() {
        // Nothing to do. 
        return template.clone(); 
    }

    let mut processed_template = template.clone();
    let mut _match = regex.find(&template);
    while _match.is_some() {
        let template_start: String = String::from(&processed_template[.._match.unwrap().start()]);
        let template_end:   String = String::from(&processed_template[_match.unwrap().end()..]);

        let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), &file_name);

        processed_template = template_start + &replacement_symbol + &template_end;

        _match = regex.find(&processed_template);
    }

    processed_template
}

pub fn create_replacement_value(token: &str, file_name: &String) -> String {

    println!("Replacing token {:?}", token);

    match token {
        "[]FILE_NAME[]"         => { return file_name.clone(); }
        "[]FILE_NAME_AS_TYPE[]" => { return create_type_from_file_name(file_name); },
        &_ => {}
    }

    String::from("")
}

pub fn create_type_from_file_name(file_name: &String) -> String {
    file_name.clone()
}

