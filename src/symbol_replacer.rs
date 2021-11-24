use regex::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;

pub struct ProcessedTemplateFile {
    pub data: String
}

impl ProcessedTemplateFile {
    pub fn new(unprocessed: &UnprocessedTemplateFile, file_description: &OutputFileDescription) -> Self {
        let regex = Regex::new("\\[\\][A-Z_]+\\[\\]").unwrap();

        let possible_matches = regex.find(&unprocessed.template_file_data);
        if possible_matches.is_none() {
            // Nothing to do. 
            return ProcessedTemplateFile {
                data: unprocessed.template_file_data.clone() 
            }
        }

        let mut processed_template = unprocessed.template_file_data.clone();
        let mut _match = regex.find(&unprocessed.template_file_data);
        while _match.is_some() {
            let template_start: String = String::from(&processed_template[.._match.unwrap().start()]);
            let template_end:   String = String::from(&processed_template[_match.unwrap().end()..]);

            let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), file_description);

            processed_template = template_start + &replacement_symbol + &template_end;

            _match = regex.find(&processed_template);
        }

        ProcessedTemplateFile {
            data: processed_template
        }
    }
}

pub fn replace_symbols(unprocessed_file: &UnprocessedTemplateFile, output_file_description: &OutputFileDescription) -> String {
    let regex = Regex::new("\\[\\][A-Z_]+\\[\\]").unwrap();

    let possible_matches = regex.find(&unprocessed_file.template_file_data);
    if possible_matches.is_none() {
        // Nothing to do. 
        return unprocessed_file.template_file_data.clone(); 
    }

    let mut processed_template = unprocessed_file.template_file_data.clone();
    let mut _match = regex.find(&unprocessed_file.template_file_data);
    while _match.is_some() {
        let template_start: String = String::from(&processed_template[.._match.unwrap().start()]);
        let template_end:   String = String::from(&processed_template[_match.unwrap().end()..]);

        let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), output_file_description);

        processed_template = template_start + &replacement_symbol + &template_end;

        _match = regex.find(&processed_template);
    }

    processed_template
}

pub fn create_replacement_value(token: &str, output_file_description: &OutputFileDescription) -> String {

    match token {
        "[]FILE_NAME[]"         => { return output_file_description.name_with_extension(); }
        "[]FILE_NAME_AS_TYPE[]" => { return create_type_from_file_name(&output_file_description.name_expanded_with_enumerations()); },
        "[]PARTNER_FILE[]"      => { 
            if output_file_description.extension == "h" {
                return output_file_description.name_expanded_with_enumerations() + ".cpp"; 
            } else if output_file_description.extension == "c" || output_file_description.extension == "cpp"{
                return output_file_description.name_expanded_with_enumerations() + ".h";
            } else {
                return String::from("NO PARTNER FILE");
            }
        }
        "[]EXTENSION[]"           => { return output_file_description.extension.clone(); },
        "[]PARENT_DIR[]"          => { return String::from("[]UNIMPLEMENTED[]"); },
        "[]PARENT_DIR_AS_TYPE[]"  => { return String::from("[]UNIMPLEMENTED[]"); },
        "[]CURRENT_DATE[]"        => { return get_current_date(); },
        "[]CURRENT_TIME[]"        => { return get_current_time(); },
        "[]PLATFORM[]"            => { return replace_if_not_none("[]PLATFORM[]",    &output_file_description.platform);    },
        "[]LANGUAGE[]"            => { return replace_if_not_none("[]LANGUAGE[]",    &output_file_description.language);    },
        "[]ENUMERATION[]"         => { return replace_if_not_none("[]ENUMERATION[]", &output_file_description.enumeration); },
        "[]USER[]"                => { return String::from("[]UNIMPLEMENTED[]"); }
        _ => {
            let replacement_string = create_replacement_value_that_has_variable(token);
            if replacement_string.is_some() {
                return replacement_string.unwrap();
            }
        }
    }

    println!("No match for token {:}", token);
    String::from("ERR")
}

pub fn create_replacement_value_that_has_variable(token: &str) -> Option<String> {
    None
}

fn replace_if_not_none(default: &str, replacement_val: &Option<String>) -> String {
    if replacement_val.is_none() {
        return String::from(default);
    }

    replacement_val.clone().unwrap()
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

pub fn get_current_time() -> String {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    local.time().format("%H:%M").to_string()
}

pub fn get_current_date() -> String {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    local.date().format("%m-%d-%Y").to_string()
}