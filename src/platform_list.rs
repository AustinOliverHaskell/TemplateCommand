use std::fs::File;
use std::io::Write;
use std::io::Read;

use regex::Regex;

pub struct PlatformList {
    pub platforms: Vec<String>
}

impl PlatformList {
    pub fn load(path: &String) -> Self {

        let platform_list: Vec<String>;
        let mut possible_file = File::open(path);
        if possible_file.is_err() {

            println!("No platform list found at {:}. Attempting to create one with default settings.", path);

            possible_file = File::create(path);
            match possible_file {
                Err(e) => {
                    println!("Failed to create platform list. Reason {:}", e);
                    return PlatformList {
                        platforms: Vec::new()
                    }
                },
                _ => {}
            }

            let mut file = possible_file.unwrap();
            match file.write_all(&(String::from("linux,windows").as_bytes())) {
                Err(e) => println!("Failed to create default platform list. Reason {:}", e),
                _ => {}
            }

            platform_list = vec![String::from("linux"), String::from("windows")]
        } else {
            let mut file_contents: String = String::new();
            match possible_file.unwrap().read_to_string(&mut file_contents) {
                Err(e) => println!("Failed to read file contents. Reason {:}", e),
                _ => {}
            }

            platform_list = PlatformList::parse_platforms(&file_contents);
        }

        PlatformList {
            platforms: platform_list
        }
    }

    pub fn parse_platforms(file_contents: &String) -> Vec<String>{
        let regex = Regex::new("[A-z]*").unwrap();
        
        let mut platform_list: Vec<String> = Vec::new();

        for _match in regex.captures_iter(file_contents) {
            let platform_text = _match.get(0).unwrap().as_str();
            platform_list.push(String::from(platform_text));
        }

        platform_list
    }
}