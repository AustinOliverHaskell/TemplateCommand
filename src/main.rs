mod program_args;
mod file_manip;
mod symbol_replacer;
mod platform_list;

use program_args::*;
use file_manip::*;
use symbol_replacer::*;
use platform_list::PlatformList;

fn main() {

    #[cfg(windows)]
    let path_list: Vec<String> = vec![
        String::from("C://.templates/")
    ];

    #[cfg(linux)]
    let path_list: Vec<String> = vec![
        String::from("/home/austinhaskell/.templates/")
    ];

    let mut exe_path_buff = std::env::current_exe().unwrap();
    let _ = exe_path_buff.pop();
    let _ = exe_path_buff.push("platform_list.txt");
    let platform_list_location: String = exe_path_buff.into_os_string().into_string().unwrap();

    let args = ProgramArguments::create();

    if args.verbose_output {
        println!("Using verbose output. ");

        if args.use_explicit_template_file {
            println!("Searching for template file {:?}", args.template_file);
        } else {
            println!("Searching for template file tt.{:}", args.extention);
        }
    }

    struct TemplateAndFilenamePair {
        extention: String,
        file_name_without_extention: String,
        file_name: String,
        template_file_name: String
    }

    let mut template_list: Vec<TemplateAndFilenamePair> = Vec::new();

    let template_file_name: String;
    if args.use_explicit_template_file {
        template_file_name = args.template_file.clone();
    } else {
        template_file_name = format!("tt.{:}", args.extention);
    }

    template_list.push( 
        TemplateAndFilenamePair {
            extention: args.extention.clone(),
            file_name_without_extention: args.file_name_without_extention.clone(),
            file_name: args.file_name.clone(),
            template_file_name: template_file_name
        }
    );

    if args.create_matching_header_and_source && 
      (args.extention == "cpp" || args.extention == "h") {

        let extention: String;
        if args.extention == "cpp" {
            extention = String::from("h");
        } else {
            extention = String::from("cpp");
        }

        let template_file_name: String;
        if args.use_explicit_template_file {
            template_file_name = args.template_file.clone();
        } else {
            template_file_name = format!("tt.{:}", extention);
        }

        template_list.push( 
            TemplateAndFilenamePair {
                extention: extention.clone(),
                file_name_without_extention: args.file_name_without_extention.clone(),
                file_name: args.file_name_without_extention.clone() + "." + &extention,
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
            let platform_list = PlatformList::load(&platform_list_location);

            for platform in platform_list.platforms {
                output_file_list.push(template.file_name_without_extention.clone() + "_" + &platform + "." + &template.extention);
            }
        } else {
            output_file_list.push(template.file_name.clone());
        }

        for file in output_file_list {
            if args.verbose_output {
                println!("Outputting file -> {:}", file);
            }

            let final_file = replace_symbols(&template_file, &file, &template.file_name_without_extention, &template.extention);

            write_file(&file, &final_file, args.verbose_output, args.overwrite);
        }
    }
}



