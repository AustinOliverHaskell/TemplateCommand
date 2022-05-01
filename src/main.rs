mod program_args;
mod replacement_token;
mod file_manip;
mod symbol_replacer;
mod template_file_list;
mod platform_specific;
mod command_line_documentation;
mod output_file_description;
mod util;
mod formatter;
mod file_harvester;
mod config;
mod logger;

use program_args::*;
use file_manip::*;
use symbol_replacer::*;
use template_file_list::*;
use output_file_description::*;
use command_line_documentation::print_all_variables;
use file_harvester::harvest_files_from_dir_as_string;
use platform_specific::PLATFORM_SEPARATOR_SLASH;
use config::Config;
use logger::Logger;

use log::*;

fn main() {

    let args = ProgramArguments::create();

    match log::set_boxed_logger(Box::new(Logger::new(args.verbose_output))) {
        Ok(()) => {},
        _ => {println!("Failed to initialize logging framework..."); return; } 
    }
    log::set_max_level(if args.verbose_output { LevelFilter::Info } else { LevelFilter::Warn });

    let mut exe_path_buff = std::env::current_exe().unwrap();
    let _ = exe_path_buff.pop();
    let exe_location:      String = exe_path_buff.into_os_string().into_string().unwrap();
    let template_dir_path: String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "templates";
    let config_path:       String = exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "config";

    let mut config = Config::load(&config_path);
    if config.is_err() {
        error!("Failed to load configuration file. Creating default one. ");
        let defualt_config = Config::default();
        let _ = defualt_config.write(&(exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "config"));
        config = Ok(defualt_config);
    }
    let config = config.unwrap();

    if args.verbose_output {
        info!("Program Args: {:?}", &args);
    }

    if args.show_documentation {
        print_all_variables();
        return;
    }

    if args.show_templates {
        println!("Looking for template files in {{{:}}}", template_dir_path);
        let file_list = harvest_files_from_dir_as_string(&Some(template_dir_path), &Vec::new(), false);
        println!("Found the following templates: ");
        println!("{:}", file_list);
        return;
    }

    if args.verbose_output {
        info!("Using verbose output. ");
    }

    if args.verbose_output {
        info!("Configuration being used: {:?}", config);
    }

    if args.create_blank.is_some() {
        write_file(&args.create_blank.unwrap(), &String::from(""), args.overwrite);
        return;
    }

    let template_file = UnprocessedTemplateFile::new(&args.extension_list, &template_dir_path);
    if template_file.is_none() {
        return;
    }
    let template_file = template_file.unwrap();

    let platform_list:    Vec<String>;
    if args.create_one_per_platform {
        platform_list = config.platform_list.clone();
    } else {
        platform_list = Vec::new();
    }

    let language_list:    Vec<String>;
    if args.create_one_per_language {
        language_list = config.language_list.clone();
    } else {
        language_list = Vec::new();
    }

    let enumeration_list: Vec<String>;
    if args.create_one_per_enumeration {
        enumeration_list = config.enumeration_list.clone();
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
        let processed_file = replace_symbols(&template_file, &file, &args.harvest_directory, &config.user_variables);

        let file_name;
        if args.file_has_no_extension {
           file_name = file.name_expanded_with_enumerations();
        } else {
           file_name = file.name_with_extension();
        }

        if args.write_file_to_screen {
            println!("----- {:} -----", file_name);
            println!("{:}", processed_file);
        } else {    
            write_file(&file_name, &processed_file, args.overwrite);
        }
    }
}