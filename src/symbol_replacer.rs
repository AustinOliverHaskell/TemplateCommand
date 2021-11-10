use regex::*;

pub fn replace_symbols(template: &String, file_name: &String, file_name_without_extention: &String, extention: &String) -> String {
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

        let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), &file_name, file_name_without_extention, extention);

        processed_template = template_start + &replacement_symbol + &template_end;

        _match = regex.find(&processed_template);
    }

    processed_template
}

pub fn create_replacement_value(token: &str, file_name: &String, file_name_without_extention: &String, extention: &String) -> String {

    match token {
        "[]FILE_NAME[]"         => { return file_name.clone(); }
        "[]FILE_NAME_AS_TYPE[]" => { return create_type_from_file_name(file_name_without_extention); },
        "[]PARTNER_FILE[]"      => { 
            if extention == "h" {
                return file_name_without_extention.clone() + ".cpp"; 
            } else {
                return file_name_without_extention.clone() + ".h";
            }
        }
        &_ => {}
    }

    println!("No match for token {:}", token);
    String::from(token)
}

pub fn create_type_from_file_name(file_name: &String) -> String {

    let mut type_name: String = String::new();

    let mut was_last_character_a_underscore = false;
    // Whew! This is a total mess - Austin Haskell
    type_name.push(file_name.chars().next().unwrap().to_uppercase().next().unwrap());
    for character in file_name[1..].chars() {

        if character == '_' {
            was_last_character_a_underscore = true;
            continue;
        } else {
            if was_last_character_a_underscore {
                type_name.push(character.to_uppercase().next().unwrap());
            } else {
                type_name.push(character);
            }

            was_last_character_a_underscore = false;
        }
    }

    type_name
}

