use std::collections::HashMap;

use regex::*;
use log::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;
use crate::file_manip::{get_current_path, get_current_dir_name};
use crate::util::*;
use crate::formatter::*;
use crate::file_harvester::*;

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
    token: &str, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String> ) -> String {

    info!("Matching against token: {:}", token);

    match token {
        "[]FILE_NAME[]"         => { return output_file_description.name_with_extension(); }
        "[]FILE_NAME_AS_TYPE[]" => { return string_in_pascal_case(&output_file_description.name_expanded_with_enumerations()); },
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
        "[]DIR_AS_TYPE[]"         => { return string_in_pascal_case(&get_current_dir_name().unwrap_or(String::from(""))); },
        "[]PWD[]"                 => { return get_current_path().unwrap_or(String::from("")); },
        "[]CURRENT_DATE[]"        => { return get_current_date("%m-%d-%Y"); },
        "[]CURRENT_TIME[]"        => { return get_current_time("%H:%M"); },
        "[]PLATFORM[]"            => { return replace_if_not_none("[]PLATFORM[]",    &output_file_description.platform);    },
        "[]LANGUAGE[]"            => { return replace_if_not_none("[]LANGUAGE[]",    &output_file_description.language);    },
        "[]ENUMERATION[]"         => { return replace_if_not_none("[]ENUMERATION[]", &output_file_description.enumeration); },
        "[]USER[]"                => { return whoami::username(); },
        "[]OS[]"                  => { return whoami::distro(); },
        "[]DEVICE_NAME[]"         => { return whoami::devicename(); },
        "[]VERSION[]"             => {return String::from(env!("CARGO_PKG_VERSION")); },
        _ => {
            let replacement_string = create_replacement_value_that_has_variable(token, harvest_location, output_file_description, user_variable_map);
            if replacement_string.is_some() {
                return replacement_string.unwrap();
            }
        }
    }

    error!(
        "No match for token {:}, putting 'ERR' in it's place. If you're attempting to use a variable that takes an argument, make sure that the argument and/or '{{}}' is present.", 
        token);

    String::from("ERR")
}

pub fn create_replacement_value_that_has_variable(
    token: &str, 
    harvest_location: &Option<String>, 
    output_file_description: &OutputFileDescription, 
    user_variable_map: &HashMap<String, String>) -> Option<String> {

    // @Optimize - Don't compile this regex every time this function is called. Make this a static. 
    let regex = Regex::new(r"\[\]([A-z]*)\{(.*)\}\[\]").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(token).collect();

    if capture_groups.is_empty() {
        return None;
    }

    let capture = &capture_groups[0];
    let token_name    = capture.get(1).map_or("ERR", |t| t.as_str());
    let variable_text = capture.get(2).map_or("ERR", |v| v.as_str());

    info!("Found variable expression with name {:}, and value {:}", token_name, variable_text);

    match token_name {
        "CURRENT_DATE"         => { Some(get_current_date(variable_text)) },
        "CURRENT_TIME"         => { Some(get_current_time(variable_text)) },
        "PARENT_DIR"           => { Some(String::from("UNIMPLEMENTED")) },
        "EACH_FILE_IN_DIR"     => { Some(harvest_files_from_dir_as_string(harvest_location, &parse_csv_list(variable_text), harvest_location.is_some())) },
        "FOR_EACH_FILE_IN_DIR" => { create_replacement_value_for_harvest_variable(variable_text, harvest_location) },
        "REPEAT_X_TIMES"       => { Some(String::from("UNIMPLEMENTED")) }, 
        "USER_VAR"             => { user_variable(variable_text, user_variable_map) }, 
        "FILE_NAME_AS_TYPE"    => { file_name_as_type_with_args(&output_file_description.name_expanded_with_enumerations(), variable_text) },
        "IMPORT"               => { import_file(variable_text)},
        "BANNER"               => { create_banner(variable_text, output_file_description, harvest_location, user_variable_map) },
        "FILE_NAME"            => { file_name_with_args(&output_file_description.name_expanded_with_enumerations(), variable_text, &output_file_description.extension)},
        "ERR"                  => None,
        _ => None,
    }
}

fn create_replacement_value_for_harvest_variable(parameters: &str, harvest_location: &Option<String>) -> Option<String>{

    let parameter_list: Vec<&str> = parameters.split("|||").collect();
    if parameter_list.len() != 2 {
        error!("Incorrect number of arguments to FOR_EACH_FILE_IN_DIR, expected both an ignore list and the line you wish to repeat. If you have no files you want to ignore then leave it blank, but it must be included. ");
        return None;
    }

    let ignore_list = parse_csv_list(parameter_list[0]);
    for item in &ignore_list {
        info!("Ignoring file type/name: {:?}", item);
    }

    let harvested_files = harvest_files_from_dir(harvest_location, &ignore_list);

    let user_line_parameter = parameter_list[1];

    let mut replacement_value: String = String::new();
    for file in harvested_files {
        replacement_value += &(replace_harvest_variables(user_line_parameter, file) + PLATFORM_LINE_ENDING); 
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

fn file_name_as_type_with_args(name: &str, variable: &str) -> Option<String> {

    let first_char = variable.chars().nth(0);
    if first_char.is_none() {
        warn!("No variable defined in FILE_NAME_AS_TYPE yet brackets exist. Remove the brackets or add a variable.");
        return None;
    }
    let first_char = first_char.unwrap();
    if first_char == '-' {
        info!("Subtracting endings. ");

        // @future: make this also take a formatting argument. 
        let formatted_string = subtract_ending_off_string(&string_in_pascal_case(name), &variable[1..]);
        if formatted_string.is_err() {
            error!("Failed to subtract ending {{{:}}}. Reason: {:}", &variable[1..], formatted_string.unwrap_err());
            return None;
        }

        return Some(formatted_string.unwrap());

    } else if first_char == '+' {
        info!("Appending endings. ");

        // @future: make this also take a formatting argument. 
        return Some(
            string_in_pascal_case(&name.to_string()) + 
            &variable[1..]
        ); 
    } else {
        return match variable {
            "caps"   => { Some(string_in_all_caps(&String::from(name))) },
            "lower"  => { Some(string_in_all_lowercase(name)) },
            "spaced" => { Some(string_split_into_spaces(name)) },
            "pascal" => { Some(string_in_pascal_case(name)) },
            "camel"  => { Some(string_in_camel_case(name)) }, 
            "kabob"  => { Some(string_in_kebob_case(name)) }, 
            _ => {
                error!("No recognized formatting method for {{{:}}}. Check documentation for valid formatting methods. ", variable);
                None
            }
        }
    }
}

fn file_name_with_args(name: &str, variable: &str, extension: &str) -> Option<String> {

    let first_char = variable.chars().nth(0);
    if first_char.is_none() {
        warn!("No variable defined in FILE_NAME yet brackets exist. Remove the brackets or add a variable.");
        return None;
    }
    let first_char = first_char.unwrap();
    if first_char == '-' {
        info!("Subtracting endings. ");
        
        // @future: make this also take a formatting argument. 
        let formatted_string = subtract_ending_off_string(&name, &variable[1..]);
        if formatted_string.is_err() {
            error!("Failed to subtract ending {{{:}}}. Reason: {:}", &variable[1..], formatted_string.unwrap_err());
            return None;
        }

        return Some(formatted_string.unwrap() + "." + extension);

    } else if first_char == '+' {
        info!("Appending endings. ");

        // @future: make this also take a formatting argument. 
        return Some(
            string_in_pascal_case(&name.to_string()) + 
            &variable[1..] + "." + extension
        ); 
    } else {
        error!("Got unknown variable with FILE_NAME{{}}, only supported actions are +/- endings");
    }

    None
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
    variable: &str, 
    output_file_description: &OutputFileDescription, 
    harvest_location: &Option<String>, 
    user_variable_map: &HashMap<String, String>) -> Option<String> {

    // Not using split_once since it's still marked experimental. 
    let parameter_list: Vec<&str> = variable.split("|||").collect();

    if parameter_list.len() < 2 {
        error!("Banner variable supplied with insufficient parameters.");
        return None;
    }

    info!("Parameter list for banner is {:?}", parameter_list);


    let mut combined_params: String = parameter_list[1].to_string();
    if parameter_list.len() >= 3 {
        for i in 2..parameter_list.len() {
            combined_params += parameter_list[i];
        }
    }

    let mut banner_symbol = parameter_list[0];
    let raw_message: String = combined_params;
    if parameter_list[0] == "" {
        banner_symbol = "*";
    }

    let message: String = replace_sub_symbols(&raw_message, output_file_description, harvest_location, user_variable_map);

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