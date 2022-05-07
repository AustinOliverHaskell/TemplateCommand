// This file contains all the functions used to convert to CAPS, Pascal Case, and Camel Case. 
use log::*;

pub fn string_in_pascal_case(string: &str) -> String {

    let mut type_name: String = String::new();

    let mut was_last_character_a_underscore = false;
    // Whew! This is a total mess - Austin Haskell
    type_name.push(string.chars().next().unwrap().to_uppercase().next().unwrap());
    for character in string[1..].chars() {

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

pub fn string_in_all_caps(string: &str) -> String {
    let mut name = string.to_string();
    name.make_ascii_uppercase();
    name
}

pub fn string_in_all_lowercase(string: &str) -> String {
    let formatted_string = string.clone();
    formatted_string.to_lowercase()
}

// @future - Make this split caps into two strings too. ie: "BigWords" would become "big words"
pub fn string_split_into_spaces(string: &str) -> String {
    let mut formatted_string = String::new();

    let raw_string = string.to_string();
    let split: Vec<&str> = raw_string.split("_").collect();
    for index in 0..split.len()-1 {
        formatted_string += split[index];
        formatted_string += " ";
    }
    formatted_string += split[split.len() - 1];

    formatted_string
}

pub fn string_in_camel_case(string: &str) -> String {
    // Taken from stackoverflow...
    let pascal_case = string_in_pascal_case(string);
    let mut c = pascal_case.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }

}

pub fn string_in_kebob_case(string: &str) -> String {
    let mut formatted_string = String::new();

    let raw_string = string.to_string();
    let split: Vec<&str> = raw_string.split("_").collect();
    for index in 0..split.len()-1 {
        formatted_string += split[index];
        formatted_string += "-";
    }
    formatted_string += split[split.len() - 1];

    formatted_string
}

pub fn subtract_ending_off_string(base: &str, ending: &str) -> Result<String, String> {
    if base.len() < ending.len() {
        warn!("Trying to remove ending that doesn't exist -{:}", ending);
        return Err("Subtracting an ending that is longer than the base. ".to_string());
    }

    let ending_start = base.len() - ending.len();
    let ending_to_replace = &base[ending_start..];

    if ending_to_replace == ending {
        return Ok((&base[..ending_start]).to_string());
    } else {
        warn!("Trying to remove ending that doesn't exist -{:}", ending);
        return Err("Subtracting an ending that doesn't exist. ".to_string());
    }
}

pub fn format_append_and_remove(string: &str, operations: &Vec<String>) -> Option<String> {
    if operations.is_empty() {
        error!("- INTERNAL - Got an empty list of operations for format_append_and_remove. ");
        return None;
    }

    info!("Got operations list of: {:?}", operations);

    let mut formatted_string = string.to_string();
    for operation in operations {
        let variable = operation.trim();
    
        let first_char = variable.chars().nth(0).unwrap();
        if first_char == '-' {    
            let subtracted_string = subtract_ending_off_string(&formatted_string, &variable[1..]);
            match subtracted_string {
                Ok(subtracted_string) => formatted_string = subtracted_string,
                Err(_) => return Some(formatted_string)
            }
        } else if first_char == '+' {    
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

pub fn format_append_and_remove_but_ensure_formatted_to_type(string: &str, operations: &Vec<String>, default_formatting: String) -> Option<String> {
    let mut operations = operations.clone();
    operations.insert(0, default_formatting);
    format_append_and_remove(string, &operations)
}