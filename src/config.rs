use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Write;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
	pub enumeration_list: Vec<String>,
	pub language_list: Vec<String>,
	pub platform_list: Vec<String>,
	pub user_variables: HashMap<String, String>
}

impl Config {
	pub fn load(path: &str) -> Result<Self, String> {
		let raw_config = read_to_string(path);
		let config: Self; 
		match raw_config {
			Ok(val) => config = serde_json::from_str(&val).unwrap(),
			_ => return Err("Failed to load configuration file. ".to_string())
		}

		return Ok(config);
	}

	pub fn write(self: &Self, path: &str) -> Result<(), String> {

		let arg_dump = serde_json::to_string(&self);
        match arg_dump {
            Err(e) => {
                return Err("Error: Failed to dump arguments to file. ".to_string());
            },
            _ => { }
        }

        let raw_arg_file = File::create(path);
        let mut arg_file;
        match raw_arg_file {
            Err(e) => {
                return Err(format!("Failed to create/open argument file: {:?}. -- Detailed error: {:}", path, e).to_string());
            },
            Ok(val) => arg_file = val
        }
        arg_file.write_all(arg_dump.unwrap().as_bytes()).unwrap();

		Ok(())
	} 

	pub fn default() -> Self {
		Config {
			enumeration_list: vec!["a".to_string(), "b".to_string()],
			language_list: vec!["en".to_string(), "fr".to_string()],
			platform_list: vec!["windows".to_string(), "linux".to_string(), "mac_os".to_string()],
			user_variables: HashMap::new()
		}
	}
}
