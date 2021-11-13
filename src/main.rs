mod program_args;
mod file_manip;
mod symbol_replacer;
mod enumeration_list;
mod template_file_list;

use program_args::*;
use file_manip::*;
use symbol_replacer::*;
use enumeration_list::*;

fn main() {

    let default_platform_list:    Vec<String> = vec![String::from("windows"), String::from("linux")];
    let default_language_list:    Vec<String> = vec![String::from("en"),      String::from("fr")];
    let default_enumeration_list: Vec<String> = vec![String::from("a"),       String::from("b")];

    #[cfg(windows)]
    let path_list: Vec<String> = vec![
        String::from("C://.templates/")
    ];

    #[cfg(target_os = "linux")]
    let path_list: Vec<String> = vec![
        String::from("/home/austinhaskell/.templates")
    ];

    let mut exe_path_buff = std::env::current_exe().unwrap();
    let _ = exe_path_buff.pop();
    let _ = exe_path_buff.push("platform_list.txt");
    let list_locations: String = exe_path_buff.into_os_string().into_string().unwrap();

    let args = ProgramArguments::create();

    if args.verbose_output {
        println!("Using verbose output. ");

        if args.use_explicit_template_file {
            println!("Searching for template file {:?}", args.template_file);
        } else {
            println!("Searching for template file tt.{:}", args.extension);
        }
    }

    struct TemplateAndFilenamePair {
        extension: String,
        file_name_without_extension: String,
        file_name: String,
        template_file_name: String
    }

    let mut template_list: Vec<TemplateAndFilenamePair> = Vec::new();

    let mut evaluated_extension: Option<String> = None;
    let mut template_file_name: Option<String> = None;
    if args.use_explicit_template_file {
        template_file_name = Some(args.template_file.clone());
    } else {
        for path in &path_list {
            // I really dont like that this is making a copy on every loop. - Austin Haskell 
            let result = find_highest_priority_extension_and_file(args.extension_list.clone(), &path);
            if result.is_none() {
                continue;
            }
    
            let (highest_priority_extension, highest_priority_template_file_name) = result.unwrap();
            template_file_name  = Some(highest_priority_template_file_name.clone());
            evaluated_extension = Some(highest_priority_extension.clone());
    
            println!("Highest Priority Extension: {:?} with file {:?}", highest_priority_extension, highest_priority_template_file_name);
        }
    }

    if template_file_name.is_none() {
        println!("Failed to resolve a valid template file for that extension. ");
        return;
    }

    template_list.push( 
        TemplateAndFilenamePair {
            extension: evaluated_extension.clone().unwrap(),
            file_name_without_extension: args.file_name_without_extension.clone(),
            file_name: args.file_name.clone(),
            template_file_name: template_file_name.unwrap()
        }
    );

    let evaluated_extension_copy = &evaluated_extension.clone().unwrap(); 
    if args.create_matching_header_and_source && 
      (evaluated_extension_copy == "cpp" || evaluated_extension_copy == "h") {

        let extension: String;
        if evaluated_extension_copy == "cpp" {
            extension = String::from("h");
        } else {
            extension = String::from("cpp");
        }

        let template_file_name: String;
        if args.use_explicit_template_file {
            template_file_name = args.template_file.clone();
        } else {
            template_file_name = format!("tt.{:}", extension);
        }

        template_list.push( 
            TemplateAndFilenamePair {
                extension: extension.clone(),
                file_name_without_extension: args.file_name_without_extension.clone(),
                file_name: args.file_name_without_extension.clone() + "." + &extension,
                template_file_name: template_file_name
            }
        );
    }

    for template in template_list {
        let possible_template = load_template_file(&path_list, &template.template_file_name, args.verbose_output);
        if possible_template.is_none() {
            println!("Failed to find template file for {:}", template.template_file_name.clone());
            return;
        }

        let template_file = possible_template.unwrap().clone();

        if args.verbose_output {
            println!("Using the following template before processing: ");
            println!(" ---------- ");
            println!("{:}", &template_file);
            println!(" ---------- ");
        }

        let mut output_file_list: Vec<String> = Vec::new();
        if args.create_one_per_platform {
            let platform_list = EnumerationList::load(&list_locations, &default_platform_list);

            for platform in platform_list.enumerations {
                output_file_list.push(template.file_name_without_extension.clone() + "_" + &platform + "." + &template.extension);
            }
        } else if args.create_one_per_enumeration {
            let enumeration_list = EnumerationList::load(&list_locations, &default_enumeration_list);

            for enumeration in enumeration_list.enumerations {
                output_file_list.push(template.file_name_without_extension.clone() + "_" + &enumeration + "." + &template.extension);
            }
        } else if args.create_one_per_language {
            let language_list = EnumerationList::load(&list_locations, &default_language_list);

            for language in language_list.enumerations {
                output_file_list.push(template.file_name_without_extension.clone() + "_" + &language + "." + &template.extension);
            }
        } 
        else {
            output_file_list.push(template.file_name.clone());
        }

        for file in output_file_list {
            if args.verbose_output {
                println!("Outputting file -> {:}", file);
            }

            let final_file = replace_symbols(&template_file, &file, &template.file_name_without_extension, &template.extension);

            if args.write_file_to_screen {
                println!("{:}", final_file);
            } else {
                write_file(&file, &final_file, args.verbose_output, args.overwrite);
            }
        }
    }
}

fn find_highest_priority_extension_and_file(extension_list: Vec<String>, path_prefix: &String) -> Option<(String, String)> {

    let mut extension_list_copy = extension_list.clone();
    let mut file_name = String::from(path_prefix) + &String::from("/tt.") + &extension_list.join(".");
    println!("Checking to see if {:} exists", file_name);

    while !check_if_file_exists(&file_name) {
        if extension_list_copy.len() == 1 {
            // No file exists, we just failed the file exists check. 
            return None;
        }

        extension_list_copy.remove(0);
        file_name = String::from(path_prefix) + &String::from("/tt.") + &extension_list_copy.join(".");

        println!("Checking to see if {:} exists", file_name);
    }

    Some((extension_list_copy.join("."), file_name))
}

