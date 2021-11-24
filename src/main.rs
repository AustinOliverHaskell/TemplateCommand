mod program_args;
mod file_manip;
mod symbol_replacer;
mod enumeration_list;
mod template_file_list;
mod platform_specific;
mod command_line_documentation;
mod output_file_description;

use program_args::*;
use file_manip::*;
use symbol_replacer::*;
use enumeration_list::*;
use template_file_list::*;
use output_file_description::*;
use command_line_documentation::print_all_variables;
use platform_specific::PLATFORM_SEPARATOR_SLASH;

fn main() {

    let default_platform_list:    Vec<String> = vec![String::from("windows"), String::from("linux")];
    let default_language_list:    Vec<String> = vec![String::from("en"),      String::from("fr")];
    let default_enumeration_list: Vec<String> = vec![String::from("a"),       String::from("b")];

    let mut exe_path_buff = std::env::current_exe().unwrap();
    let _ = exe_path_buff.pop();
    let exe_location:      String = exe_path_buff.into_os_string().into_string().unwrap();
    let template_dir_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "templates";

    let args = ProgramArguments::create();

    if args.show_documentation {
        print_all_variables();
        return;
    }

    if args.verbose_output {
        println!("Using verbose output. ");
    }

    if args.create_blank {
        write_file(&args.file_name, &String::from(""), args.verbose_output, args.overwrite);
        return;
    }

    let template_file = UnprocessedTemplateFile::new(&args.extension_list, &template_dir_path, &args.file_name, &args.extension, args.verbose_output);
    if template_file.is_none() {
        return;
    }
    let template_file = template_file.unwrap();

    let platform_list:    Vec<String>;
    if args.create_one_per_platform {
        let platform_list_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "platform_list.txt";
        platform_list = EnumerationList::load(&platform_list_path, &default_platform_list).enumerations;
    } else {
        platform_list = Vec::new();
    }

    let language_list:    Vec<String>;
    if args.create_one_per_language {
        let language_list_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "language_list.txt";
        language_list = EnumerationList::load(&language_list_path, &default_language_list).enumerations;
    } else {
        language_list = Vec::new();
    }

    let enumeration_list: Vec<String>;
    if args.create_one_per_enumeration {
        let enumeration_list_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "enumeration_list.txt";
        enumeration_list = EnumerationList::load(&enumeration_list_path, &default_enumeration_list).enumerations;
    } else {
        enumeration_list = Vec::new();
    }

    let output_file_description = OutputFileDescription {
        name: args.file_name_without_extension.clone(),
        extension: args.extension.clone(),

        platform:    None,
        language:    None, 
        enumeration: None
    };

    let mut output_file_list: Vec<OutputFileDescription> = Vec::new();
    output_file_list.push(output_file_description.clone());

    let mut expanded_list = expand_with_enumerations(
        &output_file_list, 
        &platform_list, 
        &language_list, 
        &enumeration_list
    );

    if args.create_matching_header_and_source {
        expanded_list = expand_with_matching_files(&expanded_list); 
    }

    if args.write_names_of_files_to_screen {
        for file in expanded_list {
            println!("{:}", file.name_with_extension());
        }
        return;
    }
    
    for file in expanded_list {
        let processed_file = replace_symbols(&template_file, &file);
        if args.write_file_to_screen {
            println!("----- {:} -----", file.name_with_extension());
            println!("{:}", processed_file);
        } else {
            let file_name = file.name_with_extension();
    
            write_file(&file_name, &processed_file, args.verbose_output, args.overwrite);
        }
    }
}

fn expand_with_enumerations(
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

fn expand_with_matching_files(output_file_list: &Vec<OutputFileDescription>) -> Vec<OutputFileDescription> {
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

    let expected_output = vec![base_output_description.clone()];
    let mut expected_matching_file = base_output_description.clone();
    expected_matching_file.extension = String::from("cpp");

    let actual_output = expand_with_matching_files(&expected_output);
    expected_output.push(expected_matching_file);

    assert_eq!(actual_output, expected_output);
}