pub fn replace_if_not_none(default: &str, replacement_val: &Option<String>) -> String {
    if replacement_val.is_none() {
        return String::from(default);
    }

    replacement_val.clone().unwrap()
}

pub fn parse_csv_list(csv: &str)-> Vec<String> {

    let mut list: Vec<String> = Vec::new();

    for item in csv.split(',') {
        let item_without_whitespace = item.replace(" ", "");
        list.push(String::from(item_without_whitespace));
    }

    list
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

#[test]
fn extract_name_and_extension() {

    assert_eq!(extract_file_name_and_extension_from_path("/home/austin/test/"), None);

    assert_eq!(extract_file_name_and_extension_from_path("/home/austin/test/foo.txt").unwrap(), "foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path("home/foo.txt").unwrap(), "foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path("foo.txt").unwrap(), "foo.txt");
    assert_eq!(extract_file_name_and_extension_from_path("/home/foo.txt").unwrap(), "foo.txt"); 
    assert_eq!(extract_file_name_and_extension_from_path("/home/foo").unwrap(), "foo"); 
}