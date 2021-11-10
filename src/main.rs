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

    let template: String;
    if args.use_explicit_template_file {
        template = args.template_file.clone();
    } else {
        template = format!("tt.{:}", args.extention);
    }

    let possible_template = load_template_file(&path_list, &template, args.verbose_output);
    if possible_template.is_none() {
        println!("Failed to find template file for {:}", template.clone());
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
            output_file_list.push(args.file_name_without_extention.clone() + "_" + &platform + "." + &args.extention);
        }

    } else {
        output_file_list.push(args.file_name.clone());
    }

    for file in output_file_list {
        if args.verbose_output {
            println!("Outputting file -> {:}", file);
        }

        let final_file = symbol_replacer::replace_symbols(&template_file, &file);

        write_file(&file, &final_file, args.verbose_output, args.overwrite);
    }
}



