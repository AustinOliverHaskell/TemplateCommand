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