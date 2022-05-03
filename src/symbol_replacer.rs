use std::collections::HashMap;

use regex::*;
use log::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;
use crate::file_manip::{get_current_path, get_current_dir_name};
use crate::util::*;
use crate::formatter::*;
use crate::file_harvester::*;
use crate::replacement_token::*;

use crate::platform_specific::*;

pub fn replace_symbols(
    unprocessed_file: &UnprocessedTemplateFile, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String> ) -> String {

    // @Optimize - Don't compile this regex every time this function is called. Make this a static. 
    let regex = Regex::new(r"\[\][A-z]+(\{.*\})*\[\]").unwrap();

    let possible_matches = regex.find(&unprocessed_file.template_file_data);
    if possible_matches.is_none() {
        // Nothing to do. 
        return unprocessed_file.template_file_data.clone(); 
    }

    let processed_template = unprocessed_file.template_file_data.clone();
    replace_sub_symbols(
        &processed_template, 
        output_file_description, 
        harvest_location, 
        user_variable_map)
}

pub fn replace_sub_symbols(
    data_to_replace: &String, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String>) -> String {

    let regex = Regex::new(r"\[\][A-z]+(\{.*\})*\[\]").unwrap();

    let mut processed_template = data_to_replace.clone();
    let mut _match = regex.find(&data_to_replace);
    while _match.is_some() {
        let template_start: String = String::from(&processed_template[.._match.unwrap().start()]);
        let template_end:   String = String::from(&processed_template[_match.unwrap().end()..]);

        let replacement_symbol = create_replacement_value(_match.unwrap().as_str(), output_file_description, harvest_location, user_variable_map);

        processed_template = template_start + &replacement_symbol + &template_end;

        _match = regex.find(&processed_template);
    }

    processed_template

}

pub fn create_replacement_value(
    token_text: &str, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String> ) -> String {

    info!("Matching against token: {:}", token_text);

    let token = ReplacementToken::from_string(token_text);
    if token.is_err() {
        error!("Failed to parse token");
        return "ERR".to_string()
    }
    let token = token.unwrap();
    info!("Replacement token: {:?}", token);

    let replacement_value: Option<String>;
    if token.has_variables() {
        replacement_value = match token.id.as_ref() {
            "CURRENT_DATE"         => { Some(get_current_date(&token.get_variable_as_string(0))) },
            "CURRENT_TIME"         => { Some(get_current_time(&token.get_variable_as_string(0))) },
            "PARENT_DIR"           => { Some(String::from("UNIMPLEMENTED")) },
            "EACH_FILE_IN_DIR"     => { Some(harvest_files_from_dir_as_string(harvest_location, &token.get_variable_at(0), harvest_location.is_some())) },
            "FOR_EACH_FILE_IN_DIR" => { create_replacement_value_for_harvest_variable(&token.get_variable_at(0), &token.get_variable_as_string(1), harvest_location) },
            "REPEAT_X_TIMES"       => { Some(String::from("UNIMPLEMENTED")) }, 
            "USER_VAR"             => { user_variable(&token.get_variable_as_string(0), user_variable_map) }, 
            "FILE_NAME_AS_TYPE"    => { file_name_as_type_with_args(&output_file_description.name_expanded_with_enumerations(), &token.get_variable_at(0)) },
            "IMPORT"               => { import_file(&token.get_variable_as_string(0))},
            "BANNER"               => { create_banner(&token.get_variable_as_string(0), &token.get_variable_as_string(1), output_file_description, harvest_location, user_variable_map) },
            "FILE_NAME"            => { file_name_with_args(&output_file_description.name_expanded_with_enumerations(), &token.get_variable_at(0), &output_file_description.extension)},
            "ERR"                  => None,
            _ => None,
        }
    } else {
        replacement_value = match token.id.as_ref() {
            "FILE_NAME"         => { return output_file_description.name_with_extension(); }
            "FILE_NAME_AS_TYPE" => { return string_in_pascal_case(&output_file_description.name_expanded_with_enumerations()); },
            "FILE_NAME_IN_CAPS" => { return string_in_all_caps(&output_file_description.name_expanded_with_enumerations()); },
            "PARTNER_FILE"      => { 
                if output_file_description.extension == "h" {
                    return output_file_description.name_expanded_with_enumerations() + ".cpp"; 
                } else if output_file_description.extension == "c" || output_file_description.extension == "cpp"{
                    return output_file_description.name_expanded_with_enumerations() + ".h";
                } else {
                    return String::from("NO PARTNER FILE");
                }
            }
            "EXTENSION"           => { return output_file_description.extension.clone(); },
            "DIR"                 => { return get_current_dir_name().unwrap_or(String::from(""));},
            "DIR_AS_TYPE"         => { return string_in_pascal_case(&get_current_dir_name().unwrap_or(String::from(""))); },
            "PWD"                 => { return get_current_path().unwrap_or(String::from("")); },
            "CURRENT_DATE"        => { return get_current_date("%m-%d-%Y"); },
            "CURRENT_TIME"        => { return get_current_time("%H:%M"); },
            "PLATFORM"            => { return replace_if_not_none("[]PLATFORM[]",    &output_file_description.platform);    },
            "LANGUAGE"            => { return replace_if_not_none("[]LANGUAGE[]",    &output_file_description.language);    },
            "ENUMERATION"         => { return replace_if_not_none("[]ENUMERATION[]", &output_file_description.enumeration); },
            "USER"                => { return whoami::username(); },
            "OS"                  => { return whoami::distro(); },
            "DEVICE_NAME"         => { return whoami::devicename(); },
            "VERSION"             => {return String::from(env!("CARGO_PKG_VERSION")); },
            _ => {None}
        };
    }

    if replacement_value.is_none() {
        error!(
            "No match for token {:?}, putting 'ERR' in it's place.", 
            token);
        "ERR".to_string()
    } else {
        replacement_value.unwrap()
    }

    // @Future: Implement a Did you mean? feature. 
}

fn create_replacement_value_for_harvest_variable(include_list: &Vec<String>, user_line: &str, harvest_location: &Option<String>) -> Option<String>{

    let harvested_files = harvest_files_from_dir(harvest_location, &include_list);

    let mut replacement_value: String = String::new();
    for file in harvested_files {
        replacement_value += &(replace_harvest_variables(user_line, file) + PLATFORM_LINE_ENDING); 
    }

    Some(replacement_value)
}

fn replace_harvest_variables(line: &str, file: HarvestedFile) -> String {

    // Matches anything between two sets of {}, so {}FILE_NAME{} matches but
    //  { }FILE_NAME{dfjkasfd} will not. 
    let regex = Regex::new(r"\{\}[A-z]*\{\}").unwrap();

    let mut line_with_evaluated_variables = String::from(line);
    let mut _match = regex.find(&line_with_evaluated_variables);
    while _match.is_some() {

        let start = String::from(&line_with_evaluated_variables[.._match.unwrap().start()]);
        let end   = String::from(&line_with_evaluated_variables[_match.unwrap().end()..]);

        let variable = _match.unwrap().as_str();
        let evaluated_variable: String;
        match variable {
            "{}FILE_NAME{}" => evaluated_variable = file.to_string(),
            "{}FILE_NAME_WITHOUT_EXTENSION{}" => evaluated_variable = replace_if_not_none("", &file.file_name),
            "{}EXTENSION{}" => evaluated_variable = replace_if_not_none("", &file.extension),
            "{}FILE_NAME_AS_TYPE{}" => evaluated_variable = string_in_pascal_case(&replace_if_not_none("", &file.file_name)),
            "{}FILE_NAME_IN_CAPS{}" => evaluated_variable = string_in_all_caps(&replace_if_not_none("", &file.file_name)),
            "{}PATH{}" => evaluated_variable = replace_if_not_none("", &file.path),
            _ => {
                error!("Unknown variable {:} found when parsing. See documentation for a list of currently supported variables. ", variable);
                evaluated_variable = String::from("ERR");
            }
        }

        line_with_evaluated_variables = start + &evaluated_variable + &end;
        _match = regex.find(&line_with_evaluated_variables)
    }

    String::from(line_with_evaluated_variables)
}

fn file_name_as_type_with_args(name: &str, variables: &Vec<String>) -> Option<String> {

    if variables.is_empty() {
        error!("- INTERNAL - Got an empty list of variables for file_name_as_type_with_args or file_name_with_args");
        return None;
    }
    let mut formatted_string = name.to_string();
    for variable in variables {
        if variable == "" { continue; }
    
        let first_char = variable.chars().nth(0).unwrap();

        info!("Formatted String: {:}", formatted_string);
    
        if first_char == '-' {
            info!("Subtracting endings. ");
    
            // @future: make this also take a formatting argument. 
            let subtracted_string = subtract_ending_off_string(&formatted_string, &variable[1..]);
            if subtracted_string.is_err() {
                return Some(formatted_string);
            } else {
                formatted_string = subtracted_string.unwrap();
            }
        
        } else if first_char == '+' {
            info!("Appending endings. ");
    
            // @future: make this also take a formatting argument. 
            formatted_string += &variable[1..];
        } else {
            formatted_string = match variable.as_ref() {
                "caps"   => { string_in_all_caps(&formatted_string) },
                "lower"  => { string_in_all_lowercase(&formatted_string) },
                "spaced" => { string_split_into_spaces(&formatted_string) },
                "pascal" => { string_in_pascal_case(&formatted_string) },
                "camel"  => { string_in_camel_case(&formatted_string) }, 
                "kabob"  => { string_in_kebob_case(&formatted_string) }, 
                _ => {
                    error!("No recognized formatting method for {{{:}}}. Check documentation for valid formatting methods. ", variable);
                    return None
                }
            }
        }
    }

    Some(formatted_string)
}

fn file_name_with_args(name: &str, variables: &Vec<String>, extension: &str) -> Option<String> {
    let formatted_string = file_name_as_type_with_args(name, variables);
    if formatted_string.is_none() {
        return Some(name.to_string() + "." + extension);
    } else {
        return Some(formatted_string.unwrap() + "." + extension)
    }
}

fn user_variable(variable: &str, user_variable_map: &HashMap<String, String>) -> Option<String> {

    info!("Looking for user variable: {:}", variable);
    

    if user_variable_map.contains_key(variable) {
        let variable_value = user_variable_map[variable].clone();

        info!("Found variable {:} to have a value of {:}", variable, variable_value);

        return Some(variable_value);
    } 

    error!("No user variable with the name of {:} exists in configuration file. ", variable);

    None
}

fn import_file(variable: &str) -> Option<String> {

    use std::fs::read_to_string;

    info!("Attempting to import file: {:}", variable);

    let file_contents = read_to_string(variable);
    if file_contents.is_err() {
        error!("Failed to load file for import. Make sure that the file exists and that the path is correct. ");
        return None;
    }

    return Some(file_contents.unwrap());
}

fn create_banner(
    banner_symbol: &str, 
    banner_text: &str, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String>) -> Option<String> {

    let message: String = replace_sub_symbols(&banner_text.to_string(), output_file_description, harvest_location, user_variable_map);

    info!("Creating banner with symbol {{{:}}}, and message {{{:}}}", banner_symbol, message);

    // +4 to add an extra symbol and space to the message. 
    let mut banner: String = String::new();
    for _ in 0..message.len()+4 {
        banner += banner_symbol;
    }
    banner += PLATFORM_LINE_ENDING;

    banner = banner + banner_symbol + " " + &message + " " + banner_symbol + PLATFORM_LINE_ENDING;

    for _ in 0..message.len()+4 {
        banner += banner_symbol;
    }
    banner += PLATFORM_LINE_ENDING;

    Some(banner)
}

#[test]
fn file_name_as_type_with_subtraction() {
    let expected_string = "Builder".to_string();
    let test_string = "BuilderManager";

    assert_eq!(Some(expected_string), file_name_as_type_with_args(test_string, &vec!["-Manager".to_string()]));
}

#[test]
fn file_name_as_type_with_illegal_subtraction_returns_original_string() {
    let expected_string = "Builder".to_string();
    let test_string = "Builder";

    assert_eq!(Some(expected_string), file_name_as_type_with_args(test_string, &vec!["-Manager".to_string()]));
}

#[test]
fn file_name_as_type_with_subtraction_and_case_change() {

    let expected_string = "builder".to_string();
    let test_string = "BuilderManager";

    assert_eq!(
        Some(expected_string), 
        file_name_as_type_with_args(test_string, 
            &vec![
                "-Manager".to_string(),
                "lower".to_string()
        ]));
}

#[test]
fn file_name_as_type_with_subtraction_and_addition() {

    let expected_string = "BuilderObserver".to_string();
    let test_string = "BuilderManager";

    assert_eq!(
        Some(expected_string), 
        file_name_as_type_with_args(test_string, 
            &vec![
                "-Manager".to_string(),
                "+Observer".to_string()
        ]));
}

#[test]
fn file_name_as_type_with_subtraction_and_addition_and_case_change() {

    let expected_string = "BUILDEROBSERVER".to_string();
    let test_string = "BuilderManager";

    assert_eq!(
        Some(expected_string), 
        file_name_as_type_with_args(test_string, 
            &vec![
                "-Manager".to_string(),
                "+Observer".to_string(),
                "caps".to_string()
        ]));
}

#[test]
fn file_name_with_subtraction_and_addition_and_case_change() {

    let expected_string = "BUILDEROBSERVER.h".to_string();
    let test_string = "BuilderManager";

    assert_eq!(
        Some(expected_string), 
        file_name_with_args(test_string, 
            &vec![
                "-Manager".to_string(),
                "+Observer".to_string(),
                "caps".to_string()]
            , "h"));
}

