mod program_args;
mod file_manip;

use program_args::*;
use file_manip::*;

fn main() {

    let path_list: Vec<String> = vec![
        String::from("/home/austinhaskell/.templates/")
    ];

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
        println!("Using the following template: ");
        println!(" ---------- ");
        println!("{:}", &template_file);
    }

    

}

