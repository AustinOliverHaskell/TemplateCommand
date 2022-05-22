mod program_args;
mod token;
mod file_manip;
mod symbol_replacer;
mod template_file_list;
mod platform_specific;
mod command_line_documentation;
mod util;
mod formatter;
mod file_harvester;
mod file_context;
mod config;
mod logger;
mod parser;

use program_args::*;
use file_manip::*;
use symbol_replacer::*;
use template_file_list::*;
use file_context::*;
use command_line_documentation::print_all_variables;
use file_harvester::harvest_files_from_dir_as_string;
use platform_specific::PLATFORM_SEPARATOR_SLASH;
use config::Config;
use logger::Logger;
use util::*;

use log::*;

fn main() {

    let args = ProgramArguments::create();

    Logger::set_global_logger(args.verbose_output);

    let exe_location:      String = get_exe_directory().unwrap();
    let template_dir_path: String = get_template_directory().unwrap();
    let config_path:       String = get_config_path().unwrap();

    let mut config = Config::load(&config_path);
    info!("Configuration path: {:}", config_path);
    if config.is_err() {
        error!("Failed to load configuration file. Creating default one. ");
        let defualt_config = Config::default();
        let _ = defualt_config.write(&(exe_location.clone() + PLATFORM_SEPARATOR_SLASH + "config"));
        config = Ok(defualt_config);
    }
    let config = config.unwrap();

    info!("Program Args: {:?}", &args);

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

    info!("Using verbose output. ");
    info!("Configuration being used: {:?}", config);

    if args.shove_header.is_some() {

        let target_file = args.shove_header.unwrap();
        info!("Prepending header onto file {:}", target_file);

        let file_contents = file_manip::load_file(&"".to_string(), &target_file);
        if file_contents.is_none() {
            error!("Failed to load target file to apply header. Please make sure that the file {:} exists", target_file);
            return;
        }
        let mut file_contents = file_contents.unwrap();

        info!("Successfuly read file: {:}", target_file);

        let header_template = TemplateFile::new_header(&args.extension_list, &template_dir_path);
        if header_template.is_none() {
            return; 
        }

        let file_context = FileContext::from_full_file_path(&target_file);
        if file_context.is_none() {
            return;
        }

        let evaluated_header_template = replace_symbols(
            &header_template.unwrap(), 
            &file_context.unwrap(), 
            &None, 
            &config);

        // @future: Allow the swap of headers by adding a stop point to them. 
        file_contents = evaluated_header_template + &file_contents;

        write_file(&target_file, &file_contents, true);

        return; 
    }

    if args.create_blank.is_some() {
        write_file(&args.create_blank.unwrap(), &String::from(""), args.overwrite);
        return;
    }

    let template_file = TemplateFile::new(&args.extension_list, &template_dir_path);
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

    let output_file_description = FileContext {
        name: args.file_name_without_extension.clone(),
        extension: args.extension.clone(),
        path: String::new(),

        enumerations: FileEnumeration {
            platform:    None,
            language:    None, 
            user_defined: None
        }
    };

    let output_file_list = FileContext::enumerate(
        &output_file_description, 
        &platform_list, 
        &language_list, 
        &enumeration_list
    );

    /*if args.create_matching_header_and_source {
        expanded_list = expand_with_matching_files(&expanded_list); 
    }*/

    if args.write_names_of_files_to_screen {
        for file in output_file_list {
            println!("{:}", file.name_with_extension());
        }
        return;
    }
    
    for file in output_file_list {
        let processed_file = replace_symbols(&template_file, &file, &args.harvest_directory, &config);

        let file_name = file.name_with_extension(); 

        if args.write_file_to_screen {
            println!("----- {:} -----", file_name);
            println!("{:}", processed_file);
        } else {    
            write_file(&file_name, &processed_file, args.overwrite);
        }
    }
}