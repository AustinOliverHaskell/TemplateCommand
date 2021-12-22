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