use regex::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;
use crate::file_manip::{get_current_path, get_current_dir_name, get_all_files_in_dir, extract_file_name_and_extension_from_path};

use crate::platform_specific::*;

pub fn replace_symbols(
    unprocessed_file: &UnprocessedTemplateFile, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    be_verbose: bool) -> String {

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

        let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), output_file_description, harvest_location, be_verbose);

        processed_template = template_start + &replacement_symbol + &template_end;

        _match = regex.find(&processed_template);
    }

    processed_template
}

pub fn create_replacement_value(token: &str, output_file_description: &OutputFileDescription, harvest_location: &Option<String>, be_verbose: bool) -> String {

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
            let replacement_string = create_replacement_value_that_has_variable(token, harvest_location, be_verbose);
            if replacement_string.is_some() {
                return replacement_string.unwrap();
            }
        }
    }

    println!(
        "No match for token {:}, putting 'ERR' in it's place. If you're attempting to use a variable that takes an argument, make sure that the argument and/or '{{}}' is present.", 
        token);

    String::from("ERR")
}

pub fn create_replacement_value_that_has_variable(token: &str, harvest_location: &Option<String>, be_verbose: bool) -> Option<String> {

    // @Optimize - Don't compile this regex every time this function is called. Make this a static. 
    let regex = Regex::new(r"\[\]([A-z]*)\{(.*)\}\[\]").unwrap();

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
        "EACH_FILE_IN_DIR"     => { Some(harvest_files_from_dir_as_string(harvest_location, parse_csv_list(variable_text), be_verbose)) },
        "FOR_EACH_FILE_IN_DIR" => { create_replacement_value_for_harvest_variable(variable_text, harvest_location, be_verbose) },
        "REPEAT_X_TIMES"       => { Some(String::from("UNIMPLEMENTED")) }, 
        "USER_VAR"             => { Some(String::from("UNIMPLEMENTED")) }, 
        "ERR" => None,
        _ => None,
    }
}

fn create_replacement_value_for_harvest_variable(parameters: &str, harvest_location: &Option<String>, be_verbose: bool) -> Option<String>{

    println!("Working with variables: {:?}", parameters);

    let parameter_list: Vec<&str> = parameters.split("|||").collect();
    if parameter_list.len() != 2 {
        println!("Incorrect number of arguments to FOR_EACH_FILE_IN_DIR, expected both an ignore list and the line you wish to repeat. If you have no files you want to ignore then leave it blank, but it must be included. ");
        return None;
    }

    let ignore_list = parse_csv_list(parameter_list[0]);
    if be_verbose {
        for item in ignore_list {
            println!("Ignoring file type/name: {:?}", item);
        }
    }

    let harvested_files = harvest_files_from_dir(harvest_location, Vec::new(), be_verbose);

    let user_line_parameter = parameter_list[1];

    let mut replacement_value: String = String::new();
    for file in harvested_files {
        replacement_value += &(replace_harvest_variables(user_line_parameter, "", "") + PLATFORM_LINE_ENDING); 
    }

    Some(replacement_value)
}

fn replace_harvest_variables(line: &str, file_name_without_extension: &str, file_name_full: &str) -> String {




    String::from(line)
}

// @todo: move this harvest stuff into it's own type and file. 
fn harvest_files_from_dir_as_string(dir: &Option<String>, ignore_list: Vec<String>, be_verbose: bool) -> String {

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
fn harvest_files_from_dir(dir: &Option<String>, ignore_list: Vec<String>, be_verbose: bool) -> Vec<String>{
    Vec::new()
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

pub fn parse_csv_list(csv: &str)-> Vec<String> {

    let mut list: Vec<String> = Vec::new();

    for item in csv.split(',') {
        let item_without_whitespace = item.replace(" ", "");
        list.push(String::from(item_without_whitespace));
    }

    list
}