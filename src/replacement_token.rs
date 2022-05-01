use log::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReplacementToken {
	pub id: String,
	pub variables: Option<Vec<ReplacementVariable>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReplacementVariable {
	pub variable_list: Vec<String>
}

impl ReplacementVariable {
	pub fn from_string(text: &str) -> Self {

		let items: Vec<&str> = text.split(",").collect();
		let mut formatted_items: Vec<String> = Vec::new();

		for item in items {
			if item == "" { continue; }
			formatted_items.push(item.trim().to_string());
		}

		ReplacementVariable {
			variable_list: formatted_items
		}
	}

	// Puts all the variables in the list back together. 
	pub fn rebuild_string(self: &Self) -> String {
		if self.variable_list.len() == 1 {
			return self.variable_list[0].to_string();
		} else if self.variable_list.is_empty() {
			return "".to_string();
		} 

		let mut string: String = String::new();
		for index in 0..self.variable_list.len() - 1 {
			string += &(self.variable_list[index].to_string() + ", ");
		}
		string += &(self.variable_list[self.variable_list.len() - 1].to_string());

		string
	}
}

impl ReplacementToken {
	pub fn new(id: String, variables: Vec<ReplacementVariable>) -> Self {
		ReplacementToken {
			id: id.clone(),
			variables: Some(variables.clone())
		}
	}

	pub fn from_string(text: &str) -> Result<Self, String> {
		use regex::*;

		let identifier_regex = Regex::new(r"[A-Z_]+").unwrap();
		let variable_group_regex = Regex::new(r"\{(.*)\}").unwrap();

		let identifier_capture_groups: Vec<Captures> = identifier_regex.captures_iter(text).collect();
		let variable_capture_groups: Vec<Captures> = variable_group_regex.captures_iter(text).collect();

		if identifier_capture_groups.is_empty() {
			return Err(format!("Failed to parse identifier for line {:}", text));
		}

		let identifier = (&identifier_capture_groups[0]).get(0).map_or("ERR", |i| i.as_str());
		
		let variable_group: Option<&str>;
		if variable_capture_groups.is_empty() {
			variable_group = None;
		} else {
			let variable_text = (&variable_capture_groups[0]).get(1).map_or("ERR", |v| v.as_str());

			if variable_text == "" {
				variable_group = None;
			} else {
				variable_group = Some(variable_text);
			}
		}

		info!("Identifier: {:}", identifier);
		info!("Variable Group: {:?}", variable_group);

		let mut variables: Vec<ReplacementVariable> = Vec::new();
		if variable_group.is_some() {
			let variable_group_text = variable_group.unwrap();

			let parameter_list: Vec<&str> = variable_group_text.split("|||").collect();

			for parameter in parameter_list {
				info!("Replacement Variable Found: {:?}", ReplacementVariable::from_string(parameter));
				variables.push(ReplacementVariable::from_string(parameter))
			}
		}

		Ok(ReplacementToken {
			id: identifier.to_string(),
			variables: if variables.len() == 0 { None } else { Some(variables) }
		})
	}
}


#[test]
fn token_parse_without_variables() {

	let test_text = "FILE_NAME";

	let expected_result = ReplacementToken {
		id: "FILE_NAME".to_string(),
		variables: None
	};

	assert_eq!(ReplacementToken::from_string(test_text).unwrap(), expected_result)
}

#[test]
fn token_parse_with_variable() {
	let test_text = "FILE_NAME{-_model}";

	let expected_result = ReplacementToken {
		id: "FILE_NAME".to_string(),
		variables: Some(vec![
			ReplacementVariable {
				variable_list: vec![
					"-_model".to_string()
				]
			}
		])
	};

	assert_eq!(ReplacementToken::from_string(test_text).unwrap(), expected_result)
}

#[test]
fn token_parse_with_multiple_variables() {
	let test_text = "FOR_EACH_FILE_IN_DIR{qmldir, qrc|||Some text}";

	let expected_result = ReplacementToken {
		id: "FOR_EACH_FILE_IN_DIR".to_string(),
		variables: Some(vec![
			ReplacementVariable {
				variable_list: vec![
					"qmldir".to_string(),
					"qrc".to_string()
				]
			}, 
			ReplacementVariable {
				variable_list: vec![
					"Some text".to_string()
				]
			}, 
		])
	};

	assert_eq!(ReplacementToken::from_string(test_text).unwrap(), expected_result)
}

#[test]
fn token_parse_without_variables_but_with_brackets() {
	let test_text = "FILE_NAME{}";

	let expected_result = ReplacementToken {
		id: "FILE_NAME".to_string(),
		variables: None
	};

	assert_eq!(ReplacementToken::from_string(test_text).unwrap(), expected_result)
}

#[test]
fn rebuild_string_multiple_variables() {
	let test_text = "FOR_EACH_FILE_IN_DIR{qmldir, qrc|||Some, text}";

	let expected_result = "Some, text".to_string();

	assert_eq!(ReplacementToken::from_string(test_text).unwrap().variables.unwrap()[1].rebuild_string(), expected_result)
}

#[test]
fn rebuild_string_single_variable() {
	let test_text = "FOR_EACH_FILE_IN_DIR{qmldir, qrc|||Some}";

	let expected_result = "Some".to_string();

	assert_eq!(ReplacementToken::from_string(test_text).unwrap().variables.unwrap()[1].rebuild_string(), expected_result)
}
