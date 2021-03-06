use crate::PLATFORM_SEPARATOR_SLASH;

pub fn replace_if_not_none(default: &str, replacement_val: &Option<String>) -> String {
    if replacement_val.is_none() {
        return String::from(default);
    }

    replacement_val.clone().unwrap()
}

pub fn get_current_time(format: &str) -> String {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    local.time().format(format).to_string()
}

pub fn get_current_date(format: &str) -> String {
    use chrono::prelude::*;

    let local: DateTime<Local> = Local::now();

    local.date().format(format).to_string()
}

pub fn extract_extension_from_file_name(file: &str) -> Option<String>{
    use regex::*;

    let regex = Regex::new(r"\.(.*)").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(file).collect();

    if capture_groups.is_empty() {
        return None;
    }

    let capture = &capture_groups[0];
    let extension = capture.get(1).map_or("", |e| e.as_str());

    Some(String::from(extension))
}

// @redundant - We really don't need to have to spin up a regex instance for this. Could roll it into
//  the function extract_extension_from_file_name and have it return both halves. - Austin Haskell 
pub fn remove_extensions_from_file_name(file: &str) -> Option<String> {
    use regex::*;

    let regex = Regex::new(r"([A-z0-9]*)\.").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(file).collect();

    if capture_groups.is_empty() {
        return None;
    }

    let capture = &capture_groups[0];
    let file_name = capture.get(1).map_or("", |e| e.as_str());

    Some(String::from(file_name))
}

pub fn extract_file_name_and_extension_from_path(file: &str) -> Option<String> {
    use regex::*;

    #[cfg(unix)]
    let regex = Regex::new(r"(/*.*/)*(.*$)").unwrap();

    #[cfg(windows)]
    let regex = Regex::new(r"(\\*.*\\)*(.*$)").unwrap();

    let capture_groups: Vec<Captures> = regex.captures_iter(file).collect();

    if capture_groups.is_empty() { 
        return None;
    }

    let capture = &capture_groups[0];

    let ending = capture.get(2).map_or("", |e| e.as_str());
    if ending == "" {
        None
    } else {
        Some(String::from(ending))
    }
}

pub fn get_template_directory() -> Result<String, String> {
    let exe_location = get_exe_directory().unwrap();
    let template_dir_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "templates";

    Ok(template_dir_path)
}

pub fn get_config_path() -> Result<String, String> {
    let exe_location = get_exe_directory().unwrap();
    let config_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "config";

    Ok(config_path)
}

pub fn get_exe_directory() -> Result<String, String> {
    let mut exe_path_buff = std::env::current_exe().unwrap();
    let _ = exe_path_buff.pop();
    let exe_location:      String = exe_path_buff.into_os_string().into_string().unwrap();
    Ok(exe_location)
}

#[test]
fn extract_name_and_extension() {
    // @todo: This test doesn't pass on windows due to the / being different. - Austin Haskell 1/6/2022
    // @hack: Works cross platform now, but is a little ugly... - Austin Haskell 5/12/2022
    let mut test_string = format!(r"{:}home{:}austin{:}test{:}", 
        &PLATFORM_SEPARATOR_SLASH, 
        &PLATFORM_SEPARATOR_SLASH, 
        &PLATFORM_SEPARATOR_SLASH, 
        &PLATFORM_SEPARATOR_SLASH);
    assert_eq!(extract_file_name_and_extension_from_path(&test_string), None);

    test_string = format!(r"{:}home{:}austin{:}test{:}foo.txt",         
        &PLATFORM_SEPARATOR_SLASH, 
        &PLATFORM_SEPARATOR_SLASH, 
        &PLATFORM_SEPARATOR_SLASH, 
        &PLATFORM_SEPARATOR_SLASH);
    assert_eq!(extract_file_name_and_extension_from_path(&test_string).unwrap(), "foo.txt");
    test_string = format!(r"home{:}foo.txt", &PLATFORM_SEPARATOR_SLASH);
    assert_eq!(extract_file_name_and_extension_from_path(&test_string).unwrap(), "foo.txt");
    test_string = format!(r"foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path(&test_string).unwrap(), "foo.txt");
    test_string = format!(r"{:}home{:}foo.txt", &PLATFORM_SEPARATOR_SLASH, &PLATFORM_SEPARATOR_SLASH);
    assert_eq!(extract_file_name_and_extension_from_path(&test_string).unwrap(), "foo.txt"); 
    test_string = format!(r"{:}home{:}foo", &PLATFORM_SEPARATOR_SLASH, &PLATFORM_SEPARATOR_SLASH);
    assert_eq!(extract_file_name_and_extension_from_path(&test_string).unwrap(), "foo"); 
}

#[test]
fn extension_with_multiple_dots_extracts() {
    assert_eq!(Some(String::from("ui.qml")), extract_extension_from_file_name("file.ui.qml"));
}

#[test]
fn extension_with_single_dot_extracts() {
    assert_eq!(Some(String::from("qml")), extract_extension_from_file_name("file.qml"));
}

#[test]
fn file_name_extraction() {
    assert_eq!(Some(String::from("file")), remove_extensions_from_file_name("file.qml"));
    assert_eq!(Some(String::from("file")), remove_extensions_from_file_name("file.ui.qml"));
}