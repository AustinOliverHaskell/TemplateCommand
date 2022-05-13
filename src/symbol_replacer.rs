use std::collections::HashMap;

use log::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::file_manip::{get_current_path, get_current_dir_name};
use crate::util::*;
use crate::formatter::*;
use crate::file_harvester::*;
use crate::token::*;
use crate::parser::*;
use crate::config::Config;
use crate::file_context::*;

use crate::platform_specific::*;

pub fn replace_symbols(
    unprocessed_file: &UnprocessedTemplateFile, 
    file_context: &FileContext, 
    harvest_location: &Option<String>, 
    config: &Config) -> String {

    let possible_matches = Parser::find_first_token(&unprocessed_file.template_file_data);
    if possible_matches.is_none() {
        // Nothing to do. 
        return unprocessed_file.template_file_data.clone(); 
    }

    let processed_template = unprocessed_file.template_file_data.clone();
    replace_sub_symbols(
        &processed_template, 
        file_context, 
        harvest_location, 
        config)
}

pub fn replace_sub_symbols(
    data_to_replace: &String, 
    file_context: &FileContext, 
    harvest_location: &Option<String>, 
    config: &Config) -> String {

    let mut processed_template = data_to_replace.clone();
    let mut token = Parser::find_first_token(&processed_template);

    while token.is_some() {
        let found_token = token.unwrap();

        let template_start: String = String::from(&processed_template[..found_token.start]);
        let template_end:   String = String::from(&processed_template[found_token.end..]);

        let replacement_symbol = create_replacement_value(
            &processed_template[found_token.start..found_token.end], 
            file_context, 
            harvest_location, 
            &config);

        processed_template = template_start + &replacement_symbol + &template_end;

        token = Parser::find_first_token(&processed_template);
    }

    processed_template

}

pub fn create_replacement_value_with_parent_context(
    token_text: &str, 
    file_context: &FileContext, 
    parent_context: &FileContext,
    harvest_location: &Option<String>, 
    config: &Config) -> String {

    let token = Token::from_string(token_text);
    if token.is_err() {
        error!("Failed to parse token");
        return "ERR".to_string()
    }
    let token = token.unwrap();
    info!("Harvest token: {:?}", token);

    let replacement_value: String;
    replacement_value = match token.id.as_ref() {
        "IGNORE_HARVEST_FILE_NAME" => parent_context.name.to_string(), 
        "IGNORE_HARVEST_FILE_PATH" => "Err".to_string(), 
        _ => create_replacement_value(token_text, file_context, harvest_location, config)
    };
    

    replacement_value
}

pub fn create_replacement_value(
    token_text: &str, 
    file_context: &FileContext, 
    harvest_location: &Option<String>, 
    config: &Config) -> String {

    info!("Matching against token: {:}", token_text);
    info!("File Context: {:?}", file_context);

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
            "FOR_EACH_FILE_IN_DIR" => { for_each_file_in_dir(&token, &file_context, harvest_location, &config) },
            "REPEAT_X_TIMES"       => { Some("UNIMPLEMENTED".to_string()) }, 
            "USER_VAR"             => { user_variable(&token.get_variable_as_string(0), &config.user_variables) }, 
            "FILE_NAME_AS_TYPE"    => { file_name_as_type_with_args(&file_context.expand_with_enumerations(), &token) },
            "IMPORT"               => { import_file(&token) },
            "RELATIVE_IMPORT"      => { Some("UNIMPLEMENTED".to_string()) },         
            "BANNER"               => { create_banner(&token.get_variable_as_string(0), &token.get_variable_as_string(1), file_context, harvest_location, config) },
            "FILE_NAME"            => { file_name_with_args(&file_context.expand_with_enumerations(), &token, &file_context.extension)},
            "FILE_NAME_WITHOUT_EXTENSION" => { file_name_without_extension_with_args(&file_context.name.clone(), &token)}
            "HARVEST_SUBDIR"       => { Some("UNIMPLEMENTED".to_string()) },
            "HARVEST_EACH_SUBDIR"  => { Some("UNIMPLEMENTED".to_string()) },
            "DEFINE_TEMPLATE_VAR"  => { Some("UNIMPLEMENTED".to_string()) },
            "TEMPLATE_VAR"         => { Some("UNIMPLEMENTED".to_string()) },
            "ERR"                  =>   None,
            _                      =>   None,
        }
    } else {
        replacement_value = match token.id.as_ref() {
            "FILE_NAME"           => { Some(file_context.name_with_extension()) }
            "FILE_NAME_AS_TYPE"   => { Some(string_in_pascal_case(&file_context.expand_with_enumerations())) },
            "FILE_NAME_WITHOUT_EXTENSION" => { Some(file_context.name.clone()) }
            "PARTNER_FILE"        => { find_partner_file(&file_context, &config.partner_file_map) }, 
            "EXTENSION"           => { Some(file_context.extension.clone()) },
            "DIR"                 => { Some(get_current_dir_name().unwrap_or(String::new()))},
            "DIR_AS_TYPE"         => { Some(string_in_pascal_case(&get_current_dir_name().unwrap_or(String::new()))) },
            "PWD"                 => { Some(get_current_path().unwrap_or(String::new())) },
            "PATH"                => { Some(".".to_string())},   
            "CURRENT_DATE"        => { Some(get_current_date("%m-%d-%Y")) },
            "CURRENT_TIME"        => { Some(get_current_time("%H:%M")) },
            "PLATFORM"            => { Some(replace_if_not_none("[]PLATFORM[]",    &file_context.enumerations.platform))    },
            "LANGUAGE"            => { Some(replace_if_not_none("[]LANGUAGE[]",    &file_context.enumerations.language))    },
            "ENUMERATION"         => { Some(replace_if_not_none("[]ENUMERATION[]", &file_context.enumerations.user_defined)) },
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

fn for_each_file_in_dir(token: &Token, file_context: &FileContext, harvest_location: &Option<String>, config: &Config) -> Option<String>{

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
        let mut harvest_file_context = FileContext::blank();

        harvest_file_context.name = replace_if_not_none("", &file.file_name);
        harvest_file_context.path = replace_if_not_none("", &file.path);
        harvest_file_context.extension = replace_if_not_none("", &file.extension);

        replacement_value += &(replace_harvest_variables(&user_line, &harvest_file_context, file_context, harvest_location, config));
    }

    info!("--- Replacement Value: {:?} --- ", replacement_value);

    Some(replacement_value)
}

fn replace_harvest_variables(
    line: &str, 
    file_context: &FileContext, 
    parent_file_context: &FileContext, 
    harvest_location: &Option<String>, 
    config: &Config) -> String {
   
    // @todo: This function can go away now that the refactor is almost done 
    // @Hack: This shouldnt be getting all this stuff passed down. This will all hopefully go away with the file_context stuff being worked on. 
    let line_with_evaluated_variables = create_replacement_value_with_parent_context(
        &line.to_string(), 
        &file_context,
        parent_file_context, 
        harvest_location, config
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

fn import_file(token: &Token) -> Option<String> {

    use std::fs::read_to_string;

    if !token.has_variables() {
        error!("Import token found but no template name was found. ");
    }

    let variable = token.get_variable_as_string(0);

    let import_path = get_template_directory().unwrap() + PLATFORM_SEPARATOR_SLASH + &variable; 
    info!("Attempting to import file: {:}", import_path);


    let file_contents = read_to_string(import_path);
    if file_contents.is_err() {
        error!("Failed to load file for import. Make sure that the file exists and that the path is correct. ");
        return None;
    }

    return Some(file_contents.unwrap());
}

fn create_banner(
    banner_symbol: &str, 
    banner_text: &str, 
    file_context: &FileContext, 
    harvest_location: &Option<String>, 
    config: &Config) -> Option<String> {

    let message: String = replace_sub_symbols(&banner_text.to_string(), file_context, harvest_location, config);

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

fn find_partner_file(file_context: &FileContext, partner_file_map: &HashMap<String, String>) -> Option<String> {

    if file_context.extension == "".to_string() {
        error!("Cannot match partner file to file without extension. PARTNER_FILE token means nothing in this context. ");
        return None;
    }

    let mut partner_file_extension: Option<String> = None;
    if partner_file_map.contains_key(&file_context.extension) {
        partner_file_extension = Some(partner_file_map[&file_context.extension].clone())
    }

    if partner_file_extension.is_some() {
        info!("Matched file extension {:} to {:?} for PARTNER_FILE", &file_context.extension, partner_file_extension);
        Some(file_context.name.to_string() + "." + &partner_file_extension.unwrap())
    } else {
        warn!("No partner file defined in configuration for file {:}", file_context.extension);
        None
    }
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

