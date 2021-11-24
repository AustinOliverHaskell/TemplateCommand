#[derive(Debug, Clone, PartialEq)]
pub struct OutputFileDescription {
    pub name:                   String,
    pub extension:              String,

    pub enumeration:    Option<String>,
    pub language:       Option<String>,
    pub platform:       Option<String>,
}

impl OutputFileDescription {
    pub fn name_with_extension(self: &Self) -> String {
        String::from(self.name_expanded_with_enumerations()) + "." + &self.extension
    }

    pub fn name_expanded_with_enumerations(self: &Self) -> String {

        let mut platform_append: String = String::from("");
        if self.platform.is_some() {
            platform_append = String::from("_") + &(self.platform.as_ref().unwrap());
        }

        let mut language_append: String = String::from("");
        if self.language.is_some() {
            language_append = String::from("_") + &(self.language.as_ref().unwrap());
        }

        let mut enumeration_append: String = String::from("");
        if self.enumeration.is_some() {
            enumeration_append = String::from("_") + &(self.enumeration.as_ref().unwrap());
        }

        String::from(&self.name) + &platform_append + &language_append + &enumeration_append
    }
}

pub fn expand_with_enumerations(
    output_file_list: &Vec<OutputFileDescription>, 
    platform_list: &Vec<String>, 
    language_list: &Vec<String>, 
    enumeration_list: &Vec<String>) -> Vec<OutputFileDescription> {
    
    let mut results: Vec<OutputFileDescription>;

    let mut files_with_platform: Vec<OutputFileDescription> = Vec::new();
    for file in output_file_list {
        for platform in platform_list {
            let mut output_file = file.clone();

            output_file.platform = Some(platform.clone());

            files_with_platform.push(output_file);
        }
    }

    if files_with_platform.is_empty() {
        results = output_file_list.clone();
    } else {
        results = files_with_platform;
    }

    let mut files_with_language: Vec<OutputFileDescription> = Vec::new();
    for file in &results {
        for language in language_list {
            let mut output_file = file.clone();

            output_file.language = Some(language.clone());

            files_with_language.push(output_file);
        }
    }

    if !files_with_language.is_empty() {
        results = files_with_language;
    }

    let mut files_with_enumeration: Vec<OutputFileDescription> = Vec::new();
    for file in &results {
        for enumeration in enumeration_list {
            let mut output_file = file.clone();

            output_file.enumeration = Some(enumeration.clone());

            files_with_enumeration.push(output_file);
        }
    }

    if !files_with_enumeration.is_empty() {
        results = files_with_enumeration;
    }

    results
}

pub fn expand_with_matching_files(output_file_list: &Vec<OutputFileDescription>) -> Vec<OutputFileDescription> {
    let mut results: Vec<OutputFileDescription> = Vec::new();

    for file in output_file_list {
        results.push(file.clone());
        if file.extension == "cpp" || file.extension == "c" {
            let mut matching_file = file.clone();
            matching_file.extension = String::from("h");
            results.push(matching_file);
        } else if file.extension == "h" {
            let mut matching_file = file.clone();
            matching_file.extension = String::from("cpp");
            results.push(matching_file);
        }
    }

    results
}

#[test]
pub fn matching_file_name_generation_c_to_h() {

    let base_output_description = OutputFileDescription {
        name: String::from("my_test_file"),
        extension: String::from("h"),

        enumeration: None,
        platform: None,
        language: None
    };

    let mut expected_output = vec![base_output_description.clone()];
    let mut expected_matching_file = base_output_description.clone();
    expected_matching_file.extension = String::from("cpp");

    let actual_output = expand_with_matching_files(&expected_output);
    expected_output.push(expected_matching_file);

    assert_eq!(actual_output, expected_output);
}

#[test]
pub fn matching_file_name_generation_h_to_cpp() {

    let base_output_description = OutputFileDescription {
        name: String::from("my_test_file"),
        extension: String::from("cpp"),

        enumeration: None,
        platform: None,
        language: None
    };

    let mut expected_output = vec![base_output_description.clone()];
    let mut expected_matching_file = base_output_description.clone();
    expected_matching_file.extension = String::from("h");

    let actual_output = expand_with_matching_files(&expected_output);
    expected_output.push(expected_matching_file);

    assert_eq!(actual_output, expected_output);
}