use regex::*;

use crate::template_file_list::UnprocessedTemplateFile;
use crate::output_file_description::OutputFileDescription;
use crate::file_manip::{get_current_path, get_current_dir_name};
use crate::util::*;
use crate::formatter::{format_file_name_as_pascal_case, string_in_all_caps};
use crate::file_harvester::*;

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
        "[]FILE_NAME_AS_TYPE[]" => { return format_file_name_as_pascal_case(&output_file_description.name_expanded_with_enumerations()); },
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
        "[]DIR_AS_TYPE[]"         => { return format_file_name_as_pascal_case(&get_current_dir_name().unwrap_or(String::from(""))); },
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

    let parameter_list: Vec<&str> = parameters.split("|||").collect();
    if parameter_list.len() != 2 {
        println!("Incorrect number of arguments to FOR_EACH_FILE_IN_DIR, expected both an ignore list and the line you wish to repeat. If you have no files you want to ignore then leave it blank, but it must be included. ");
        return None;
    }

    let ignore_list = parse_csv_list(parameter_list[0]);
    if be_verbose {
        for item in &ignore_list {
            println!("Ignoring file type/name: {:?}", item);
        }
    }

    let harvested_files = harvest_files_from_dir(harvest_location, Vec::new(), be_verbose);

    let user_line_parameter = parameter_list[1];

    println!("Working with ignore list of: {:?}", &ignore_list);
    println!("Working with user line of {:?}", user_line_parameter);

    let mut replacement_value: String = String::new();
    for file in harvested_files {
        replacement_value += &(replace_harvest_variables(user_line_parameter, file) + PLATFORM_LINE_ENDING); 
    }

    Some(replacement_value)
}

fn replace_harvest_variables(line: &str, file: HarvestedFile) -> String {




    String::from(line)
}