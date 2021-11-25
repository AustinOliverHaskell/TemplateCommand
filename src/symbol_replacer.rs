use regex::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;
use crate::file_manip::{get_current_path, get_current_dir_name};

pub fn replace_symbols(unprocessed_file: &UnprocessedTemplateFile, output_file_description: &OutputFileDescription, be_verbose: bool) -> String {

    // @Optimize - Don't compile this regex every time this function is called. Make this a static. 
    let regex = Regex::new(r"\[\][A-z]+(\{.*\})*\[\]").unwrap();

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

        let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), output_file_description, be_verbose);

        processed_template = template_start + &replacement_symbol + &template_end;

        _match = regex.find(&processed_template);
    }

    processed_template
}

pub fn create_replacement_value(token: &str, output_file_description: &OutputFileDescription, be_verbose: bool) -> String {

    if be_verbose {
        println!("Matching against token: {:}", token);
    }

    match token {
        "[]FILE_NAME[]"         => { return output_file_description.name_with_extension(); }
        "[]FILE_NAME_AS_TYPE[]" => { return create_type_from_file_name(&output_file_description.name_expanded_with_enumerations()); },
        "[]FILE_NAME_IN_CAPS[]" => { return string_in_all_caps(&output_file_description.name_expanded_with_enumerations()); },
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
        "[]DIR[]"                 => { return get_current_dir_name().unwrap_or(String::from(""));},
        "[]DIR_AS_TYPE[]"         => { return create_type_from_file_name(&get_current_dir_name().unwrap_or(String::from(""))); },
        "[]PWD[]"                 => { return get_current_path().unwrap_or(String::from("")); },
        "[]CURRENT_DATE[]"        => { return get_current_date("%m-%d-%Y"); },
        "[]CURRENT_TIME[]"        => { return get_current_time("%H:%M"); },
        "[]PLATFORM[]"            => { return replace_if_not_none("[]PLATFORM[]",    &output_file_description.platform);    },
        "[]LANGUAGE[]"            => { return replace_if_not_none("[]LANGUAGE[]",    &output_file_description.language);    },
        "[]ENUMERATION[]"         => { return replace_if_not_none("[]ENUMERATION[]", &output_file_description.enumeration); },
        "[]USER[]"                => { return whoami::username(); },
        "[]OS[]"                  => { return whoami::distro(); },
        "[]DEVICE_NAME[]"         => { return whoami::devicename(); },
        _ => {
            let replacement_string = create_replacement_value_that_has_variable(token, be_verbose);
            if replacement_string.is_some() {
                return replacement_string.unwrap();
            }
        }
    }

    println!("No match for token {:}, putting 'ERR' in it's place.", token);
    String::from("ERR")
}

pub fn create_replacement_value_that_has_variable(token: &str, be_verbose: bool) -> Option<String> {

    // @Optimize - Don't compile this regex every time this function is called. Make this a static. 
    let regex = Regex::new(r"\[\](.*)\{(.*)\}\[\]").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(token).collect();

    if capture_groups.is_empty() {
        return None;
    }

    let capture = &capture_groups[0];
    let token_name    = capture.get(1).map_or("ERR", |t| t.as_str());
    let variable_text = capture.get(2).map_or("ERR", |v| v.as_str());

    if be_verbose {
        println!("Found variable expression with name {:}, and value {:}", token_name, variable_text);
    }

    match token_name {
        "CURRENT_DATE"         => { Some(get_current_date(variable_text)) },
        "CURRENT_TIME"         => { Some(get_current_time(variable_text)) },
        "PARENT_DIR"           => { Some(String::from("UNIMPLEMENTED")) },
        "FOR_EACH_FILE_IN_DIR" => { Some(String::from("UNIMPLEMENTED")) },
        "REPEAT_X_TIMES"       => { Some(String::from("UNIMPLEMENTED")) }, 
        "USER_VAR"             => { Some(String::from("UNIMPLEMENTED")) }, 
        "ERR" => None,
        _ => None,
    }
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

pub fn string_in_all_caps(file_name: &String) -> String {
    let mut name = file_name.clone();
    name.make_ascii_uppercase();
    return name;
}

pub fn get_current_time(format: &str) -> String {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    local.time().format(format).to_string()
}

pub fn get_current_date(format: &str) -> String {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    local.date().format(format).to_string()
}