use log::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileContext {
    pub name: String,
    pub extension: String,
    pub path: String,

    pub enumerations: FileEnumeration,
}

impl FileContext {
    pub fn blank() -> Self {
        Self {
            name: String::new(),
            extension: String::new(),
            path: String::new(),

            enumerations: FileEnumeration::blank()
        }
    }

    pub fn from_full_file_path(path: &str) -> Option<Self> {

        use crate::util::*;

        let file_name_and_extension = extract_file_name_and_extension_from_path(path);
        if file_name_and_extension.is_none() {
            error!("Failed to create file context from file path... extraction of file name and extension failed.");
            return None; 
        }
        let file_name_and_extension = file_name_and_extension.unwrap();
        let file_extension = extract_extension_from_file_name(&file_name_and_extension);
        let file_name = remove_extensions_from_file_name(&file_name_and_extension);
        if file_name.is_none() {
            error!("Path provided to create new file context has no file name. ");
            return None;
        }
        let file_name = file_name.unwrap();

        Some(Self {
            extension: file_extension.unwrap_or("".to_string()),
            path: path.to_string(),
            name: file_name,

            enumerations: FileEnumeration::blank()
        })
    }

    pub fn name_with_extension(self: &Self) -> String {
        String::from(self.expand_with_enumerations()) + "." + &self.extension
    }

    pub fn expand_with_enumerations(self: &Self) -> String {
         let mut platform_append: String = String::from("");
        if self.enumerations.platform.is_some() {
            platform_append = String::from("_") + &(self.enumerations.platform.as_ref().unwrap());
        }

        let mut language_append: String = String::from("");
        if self.enumerations.language.is_some() {
            language_append = String::from("_") + &(self.enumerations.language.as_ref().unwrap());
        }

        let mut enumeration_append: String = String::from("");
        if self.enumerations.user_defined.is_some() {
            enumeration_append = String::from("_") + &(self.enumerations.user_defined.as_ref().unwrap());
        }

        String::from(&self.name) + &platform_append + &language_append + &enumeration_append
    }

    pub fn enumerate(
        self: &Self, 
        platform_list: &Vec<String>, 
        language_list: &Vec<String>, 
        user_enumeration_list: &Vec<String>) -> Vec<Self> {

        let mut expanded_list: Vec<Self> = Vec::new();
        expanded_list.push(self.clone());

        let mut files_with_platform: Vec<Self> = Vec::new();
        for platform in platform_list {
            let mut new_context = self.clone();

            new_context.enumerations.platform = Some(platform.clone());

            files_with_platform.push(new_context);
        }

        if !files_with_platform.is_empty() {
            expanded_list = files_with_platform;
        }

        let mut files_with_language: Vec<Self> = Vec::new();
        for file in &expanded_list {
            for language in language_list {
                let mut new_context = file.clone();

                new_context.enumerations.language = Some(language.clone());

                files_with_language.push(new_context);
            }
        }

        if !files_with_language.is_empty() {
            expanded_list = files_with_language;
        }

        let mut files_with_enumeration: Vec<Self> = Vec::new();
        for file in &expanded_list {
            for user_enumeration in user_enumeration_list {
                let mut new_context = file.clone();

                new_context.enumerations.user_defined = Some(user_enumeration.clone());

                files_with_enumeration.push(new_context);
            }
        }

        if !files_with_enumeration.is_empty() {
            expanded_list = files_with_enumeration;
        }

        expanded_list
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileEnumeration {
    pub language: Option<String>,
    pub platform: Option<String>,
    pub user_defined: Option<String>
}

impl FileEnumeration {
    pub fn blank() -> Self {
        Self {
            language: None,
            platform: None,
            user_defined: None
        }
    }
}

#[test]
pub fn enumerate_expands_with_platform() {
    let mut expected_result: Vec<FileContext> = vec![FileContext::blank(), FileContext::blank()];

    expected_result[0].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: None,
            user_defined: None
    };

    expected_result[1].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: None,
            user_defined: None
    };

    let platform_list: Vec<String> = vec!["windows".to_string(), "linux".to_string()];

    assert_eq!(expected_result, FileContext::blank().enumerate(&platform_list, &Vec::new(), &Vec::new()));
}

#[test]
pub fn enumerate_expands_with_language() {
    let mut expected_result: Vec<FileContext> = vec![FileContext::blank(), FileContext::blank()];

    expected_result[0].enumerations = FileEnumeration {
            platform: None,
            language: Some("en".to_string()),
            user_defined: None
    };

    expected_result[1].enumerations = FileEnumeration {
            platform: None,
            language: Some("fr".to_string()),
            user_defined: None
    };

    let language_list: Vec<String> = vec!["en".to_string(), "fr".to_string()];

    assert_eq!(expected_result, FileContext::blank().enumerate(&Vec::new(), &language_list, &Vec::new()));
}

#[test]
pub fn enumerate_expands_with_user_defines() {
    let mut expected_result: Vec<FileContext> = vec![FileContext::blank(), FileContext::blank()];

    expected_result[0].enumerations = FileEnumeration {
            platform: None,
            language: None,
            user_defined: Some("a".to_string())
    };

    expected_result[1].enumerations = FileEnumeration {
            platform: None,
            language: None,
            user_defined: Some("b".to_string())
    };

    let user_list: Vec<String> = vec!["a".to_string(), "b".to_string()];

    assert_eq!(expected_result, FileContext::blank().enumerate(&Vec::new(), &Vec::new(), &user_list));
}

#[test]
pub fn enumerate_expands_with_platform_and_language() {
    let mut expected_result: Vec<FileContext> = Vec::new();
    for _ in 0..4 {
        expected_result.push(FileContext::blank());
    }

    expected_result[0].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: Some("en".to_string()),
            user_defined: None
    };

    expected_result[1].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: Some("fr".to_string()),
            user_defined: None
    };

    expected_result[2].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: Some("en".to_string()),
            user_defined: None
    };

    expected_result[3].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: Some("fr".to_string()),
            user_defined: None
    };

    let language_list: Vec<String> = vec!["en".to_string(), "fr".to_string()];
    let platform_list: Vec<String> = vec!["windows".to_string(), "linux".to_string()];

    assert_eq!(expected_result, FileContext::blank().enumerate(&platform_list, &language_list, &Vec::new()));
}

#[test]
pub fn enumerate_expands_with_platform_and_language_and_user_defined() {
    let mut expected_result: Vec<FileContext> = Vec::new();
    for _ in 0..8 {
        expected_result.push(FileContext::blank());
    }
    expected_result[0].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: Some("en".to_string()),
            user_defined: Some("a".to_string())
    };

    expected_result[1].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: Some("en".to_string()),
            user_defined: Some("b".to_string())
    };

    expected_result[2].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: Some("fr".to_string()),
            user_defined: Some("a".to_string())
    };

    expected_result[3].enumerations = FileEnumeration {
            platform: Some("windows".to_string()),
            language: Some("fr".to_string()),
            user_defined: Some("b".to_string())
    };

    expected_result[4].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: Some("en".to_string()),
            user_defined: Some("a".to_string())
    };

    expected_result[5].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: Some("en".to_string()),
            user_defined: Some("b".to_string())
    };

    expected_result[6].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: Some("fr".to_string()),
            user_defined: Some("a".to_string())
    };

    expected_result[7].enumerations = FileEnumeration {
            platform: Some("linux".to_string()),
            language: Some("fr".to_string()),
            user_defined: Some("b".to_string())
    };

    let language_list: Vec<String> = vec!["en".to_string(), "fr".to_string()];
    let platform_list: Vec<String> = vec!["windows".to_string(), "linux".to_string()];
    let user_list: Vec<String> = vec!["a".to_string(), "b".to_string()];

    assert_eq!(expected_result, FileContext::blank().enumerate(&platform_list, &language_list, &user_list));
}