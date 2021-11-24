#[derive(Debug, Clone)]
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