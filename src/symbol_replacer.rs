use std::collections::HashMap;

use log::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;
use crate::file_manip::{get_current_path, get_current_dir_name};
use crate::util::*;
use crate::formatter::*;
use crate::file_harvester::*;
use crate::token::*;
use crate::parser::*;

use crate::platform_specific::*;

pub fn replace_symbols(
    unprocessed_file: &UnprocessedTemplateFile, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String> ) -> String {

    let possible_matches = Parser::find_first_token(&unprocessed_file.template_file_data);
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


    let mut processed_template = data_to_replace.clone();
    let mut token = Parser::find_first_token(&processed_template);

    info!("====> Output File Description: {:?}", output_file_description);

    while token.is_some() {
        let found_token = token.unwrap();

        let template_start: String = String::from(&processed_template[..found_token.start]);
        let template_end:   String = String::from(&processed_template[found_token.end..]);

        let replacement_symbol = create_replacement_value(
            &processed_template[found_token.start..found_token.end], 
            output_file_description, 
            harvest_location, 
            user_variable_map);

        processed_template = template_start + &replacement_symbol + &template_end;

        token = Parser::find_first_token(&processed_template);
    }

    processed_template

}

pub fn create_replacement_value(
    token_text: &str, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String> ) -> String {

    info!("Matching against token: {:}", token_text);
    info!("OutputFileDescription: {:?}", output_file_description);

    let token = Token::from_string(token_text);
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
            "PARENT_DIR"           => { Some("UNIMPLEMENTED".to_string()) },
            "EACH_FILE_IN_DIR"     => { Some(harvest_files_from_dir_as_string(harvest_location, &token.get_variable_at(0), harvest_location.is_some())) },
            "FOR_EACH_FILE_IN_DIR" => { for_each_file_in_dir(&token, harvest_location, user_variable_map) },
            "REPEAT_X_TIMES"       => { Some("UNIMPLEMENTED".to_string()) }, 
            "USER_VAR"             => { user_variable(&token.get_variable_as_string(0), user_variable_map) }, 
            "FILE_NAME_AS_TYPE"    => { file_name_as_type_with_args(&output_file_description.name_expanded_with_enumerations(), &token) },
            "IMPORT"               => { import_file(&token.get_variable_as_string(0))},
            "BANNER"               => { create_banner(&token.get_variable_as_string(0), &token.get_variable_as_string(1), output_file_description, harvest_location, user_variable_map) },
            "FILE_NAME"            => { file_name_with_args(&output_file_description.name_expanded_with_enumerations(), &token, &output_file_description.extension)},
            "FILE_NAME_WITHOUT_EXTENSION" => { file_name_without_extension_with_args(&output_file_description.name.clone(), &token)}
            "ERR"                  =>   None,
            _                      =>   None,
        }
    } else {
        replacement_value = match token.id.as_ref() {
            "FILE_NAME"         => { Some(output_file_description.name_with_extension()) }
            "FILE_NAME_AS_TYPE" => { Some(string_in_pascal_case(&output_file_description.name_expanded_with_enumerations())) },
            "FILE_NAME_IN_CAPS" => { Some(string_in_all_caps(&output_file_description.name_expanded_with_enumerations())) },
            "FILE_NAME_WITHOUT_EXTENSION" => { Some(output_file_description.name.clone()) }
            "PARTNER_FILE"      => { 
                if output_file_description.extension == "h" {
                    Some(output_file_description.name_expanded_with_enumerations() + ".cpp")
                } else if output_file_description.extension == "c" || output_file_description.extension == "cpp"{
                    Some(output_file_description.name_expanded_with_enumerations() + ".h")
                } else {
                    Some("NO PARTNER FILE".to_string())
                }
            }
            "EXTENSION"           => { Some(output_file_description.extension.clone()) },
            "DIR"                 => { Some(get_current_dir_name().unwrap_or(String::new()))},
            "DIR_AS_TYPE"         => { Some(string_in_pascal_case(&get_current_dir_name().unwrap_or(String::new()))) },
            "PWD"                 => { Some(get_current_path().unwrap_or(String::new())) },
            "PATH"                => { Some(".".to_string())},   
            "CURRENT_DATE"        => { Some(get_current_date("%m-%d-%Y")) },
            "CURRENT_TIME"        => { Some(get_current_time("%H:%M")) },
            "PLATFORM"            => { Some(replace_if_not_none("[]PLATFORM[]",    &output_file_description.platform))    },
            "LANGUAGE"            => { Some(replace_if_not_none("[]LANGUAGE[]",    &output_file_description.language))    },
            "ENUMERATION"         => { Some(replace_if_not_none("[]ENUMERATION[]", &output_file_description.enumeration)) },
            "USER"                => { Some(whoami::username()) },
            "OS"                  => { Some(whoami::distro()) },
            "DEVICE_NAME"         => { Some(whoami::devicename()) },
            "VERSION"             => { Some(env!("CARGO_PKG_VERSION").to_string()) },
            _                     =>   None
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

fn for_each_file_in_dir(token: &Token, harvest_location: &Option<String>, user_variable_map: &HashMap<String, String>) -> Option<String>{

    let token = token.clone();

    let variable_list = token.get_variable_at(0);
    let mut include_list: Vec<String> = Vec::new();
    for item in variable_list {
        include_list.push(item.trim().to_string());
    }

    let user_line = token.variables.unwrap()[1].rebuild_string();

    let harvested_files = harvest_files_from_dir(harvest_location, &include_list);

    let mut replacement_value: String = String::new();
    for file in harvested_files {
        replacement_value += &(replace_harvest_variables(&user_line, file, harvest_location, user_variable_map));
    }

    info!("--- Replacement Value: {:?} --- ", replacement_value);

    Some(replacement_value)
}

fn replace_harvest_variables(line: &str, file: HarvestedFile, harvest_location: &Option<String>, user_variable_map: &HashMap<String, String>) -> String {
   
    // @Hack: This shouldnt be getting all this stuff passed down. This will all hopefully go away with the file_context stuff being worked on. 
    let line_with_evaluated_variables = replace_sub_symbols(
        &line.to_string(), 
        &OutputFileDescription {
            enumeration: None,
            platform: None,
            language: None,

            extension: replace_if_not_none("", &file.extension),
            name: replace_if_not_none("", &file.file_name)
        },
        harvest_location, user_variable_map
    );

    String::from(line_with_evaluated_variables)
}

fn file_name_as_type_with_args(file_name: &str, token: &Token) -> Option<String> {
    if !token.has_variables() {
        error!("- INTERNAL - Got an empty list of variables for file_name_as_type_with_args");
        None
    } else {
        // @todo: Make this use a default formatting from config file so that the user can define this. 
        match format_append_and_remove_but_ensure_formatted_to_type(
                file_name, 
                &token.variables.clone().unwrap()[0].variable_list, 
                "pascal".to_string()) {
            Some(value) => Some(value),
            None => Some(file_name.to_string())
        }
    }
}

fn file_name_without_extension_with_args(file_name: &str, token: &Token) -> Option<String> {
    if !token.has_variables() {
        error!("- INTERNAL - Got an empty list of variables for file_name_without_extension_with_args");
        None
    } else {
        match format_append_and_remove(file_name, &token.variables.clone().unwrap()[0].variable_list) {
            Some(value) => Some(value),
            None => Some(file_name.to_string())
        }
    }
}

fn file_name_with_args(file_name: &str, token: &Token, extension: &str) -> Option<String> {

    if !token.has_variables() {
        error!("- INTERNAL - Got an empty list of variables for file_name_with_args");
        None
    } else {
        match format_append_and_remove(file_name, &token.variables.clone().unwrap()[0].variable_list) {
            Some(value) => Some(value + "." + extension),
            None => Some(file_name.to_string() + "." + extension)
        }
    }
}

fn user_variable(variable: &str, user_variable_map: &HashMap<String, String>) -> Option<String> {

    info!("Looking for user variable: {:}", variable);

    if user_variable_map.contains_key(variable) {
        let variable_value = user_variable_map[variable].clone();

        info!("Found variable {:} to have a value of {:}", variable, variable_value);

        return Some(variable_value);
    } 

    warn!("No user variable with the name of {:} exists in configuration file. ", variable);

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

    assert_eq!(Some(expected_string), 
        file_name_as_type_with_args(test_string, 
        &Token {
            id: "FILE_NAME_AS_TYPE".to_string(),
            variables: Some(vec![TokenVariable {
                variable_list: vec!["-Manager".to_string()]
            }])
        }));
}

#[test]
fn file_name_as_type_with_illegal_subtraction_returns_original_string() {
    let expected_string = "Builder".to_string();
    let test_string = "Builder";


    assert_eq!(Some(expected_string), 
    file_name_as_type_with_args(test_string, 
    &Token {
        id: "FILE_NAME_AS_TYPE".to_string(),
        variables: Some(vec![TokenVariable {
            variable_list: vec!["-Manager".to_string()]
        }])
    }));
}

#[test]
fn file_name_as_type_with_subtraction_and_case_change() {

    let expected_string = "builder".to_string();
    let test_string = "BuilderManager";


    assert_eq!(Some(expected_string), 
        file_name_as_type_with_args(test_string, 
        &Token {
            id: "FILE_NAME_AS_TYPE".to_string(),
            variables: Some(vec![TokenVariable {
                variable_list: vec![
                    "-Manager".to_string(),
                    "lower".to_string()
                ]
            }])
        }));
}

#[test]
fn file_name_as_type_with_subtraction_and_addition() {

    let expected_string = "BuilderObserver".to_string();
    let test_string = "BuilderManager";


    assert_eq!(Some(expected_string), 
        file_name_as_type_with_args(test_string, 
        &Token {
            id: "FILE_NAME_AS_TYPE".to_string(),
            variables: Some(vec![TokenVariable {
                variable_list: vec![
                    "-Manager".to_string(),
                    "+Observer".to_string()
                ]
            }])
        }));
}

#[test]
fn file_name_as_type_with_subtraction_and_addition_and_case_change() {

    let expected_string = "BUILDEROBSERVER".to_string();
    let test_string = "BuilderManager";


    assert_eq!(Some(expected_string), 
        file_name_as_type_with_args(test_string, 
        &Token {
            id: "FILE_NAME_AS_TYPE".to_string(),
            variables: Some(vec![TokenVariable {
                variable_list: vec![
                    "-Manager".to_string(),
                    "+Observer".to_string(),
                    "caps".to_string()
                ]
            }])
        }));
}

#[test]
fn file_name_with_subtraction_and_addition_and_case_change() {

    let expected_string = "BUILDEROBSERVER.h".to_string();
    let test_string = "BuilderManager";

    assert_eq!(
        Some(expected_string), 
        file_name_with_args(test_string, 
        &Token {
            id: "FILE_NAME_AS_TYPE".to_string(),
            variables: Some(vec![TokenVariable {
                variable_list: vec![
                    "-Manager".to_string(),
                    "+Observer".to_string(),
                    "caps".to_string()
                ]
            }])
        }, "h"));
}

