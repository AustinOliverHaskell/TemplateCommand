use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Write;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
	pub enumeration_list: Vec<String>,
	pub language_list: Vec<String>,
	pub platform_list: Vec<String>,
	pub user_variables: HashMap<String, String>,

	pub partner_file_map: HashMap<String, String>
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
                return Err(format!("Error: Failed to dump arguments to file. -- Detailed error: {:}", e).to_string());
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
		let mut config = Config {
			enumeration_list: vec!["a".to_string(), "b".to_string()],
			language_list: vec!["en".to_string(), "fr".to_string()],
			platform_list: vec!["windows".to_string(), "linux".to_string(), "mac_os".to_string()],
			user_variables: HashMap::new(),
			partner_file_map: HashMap::new()
		};

		config.user_variables.insert("LOOPBACK_ADDR".to_string(), "127.0.0.1".to_string());
		config.user_variables.insert("VERSION_MANAGEMENT".to_string(), "git".to_string());

		config.partner_file_map.insert("h".to_string(), "cpp".to_string());
		config.partner_file_map.insert("c".to_string(), "h".to_string());
		config.partner_file_map.insert("cpp".to_string(), "h".to_string());

		config
	}
}
