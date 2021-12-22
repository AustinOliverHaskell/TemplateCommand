// This file contains all the functions used to convert to CAPS, Pascal Case, and Camel Case. 
pub fn format_file_name_as_pascal_case(file_name: &String) -> String {

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