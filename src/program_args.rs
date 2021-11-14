use clap::{Arg, App};

#[derive(Debug)]
pub struct ProgramArguments {
    pub template_file: String,
    pub file_name: String, 

    pub file_name_without_extension: String,
    pub extension_list: Vec<String>,
    pub extension: String,

    pub use_explicit_template_file: bool,

    pub create_matching_header_and_source: bool,
    pub create_one_per_platform: bool,
    pub create_one_per_enumeration: bool,
    pub create_one_per_language: bool,

    pub overwrite: bool,
    pub verbose_output: bool,

    pub write_file_to_screen: bool,
    pub write_names_of_files_to_screen: bool,

    pub show_documentation: bool,
}

impl ProgramArguments {
    pub fn create() -> Self {

        let args = App::new("Template Creation Tool")
                    .version("0.1")
                    .author("Austin Haskell")
                    .about("This application aids in the creation of files via the command line")
                    .arg(
                        Arg::with_name("template_file")    
                        .short("t")
                        .long("template")
                        .help("Creates the file with a specific named template")
                        .takes_value(true))
                    .arg(
                        Arg::with_name("file_name")
                        .short("f")
                        .long("file")
                        .help("File to create. Extension is used to select template to use unless -t flag is also present")
                        .takes_value(true)
                        .required(true))
                    .arg(
                        Arg::with_name("overwrite")
                        .short("o")
                        .long("overwrite")
                        .help("If present, will overwrite any file when encountering an already present file. "))
                    .arg(
                        Arg::with_name("platform")
                        .short("p")
                        .long("platform")
                        .help("If present, will create one file per item on the platform list"))
                    .arg(
                        Arg::with_name("enumeration")
                        .short("e")
                        .long("enumeration")
                        .help("If present, will create one file per item on the enumeration list"))
                    .arg(
                        Arg::with_name("language")
                        .short("l")
                        .long("language")
                        .help("If present, will create one file per item on the language list"))
                    .arg(
                        Arg::with_name("matching_headers")
                        .short("m")
                        .long("matching_files")
                        .help("If present and if applicable, will create a matching source/header file (C family only)"))
                    .arg(
                        Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("If present will use verbose output."))
                    .arg(
                        Arg::with_name("debug")
                        .short("d")
                        .long("debug")
                        .help("If present, will print output to the screen instead of writing to file. "))
                    .arg(
                        Arg::with_name("list_names")
                        .short("n")
                        .long("list_names")
                        .help("If present, will print the names of the output files without writing the actual files."))
                    .arg(
                        Arg::with_name("show_documentation")
                        .short("s")
                        .long("show_doc")
                        .help("(NOT IMPLEMENTED, SEE README FOR DOC) If present, will print replacement variables and an explanation of what they do. ")
                    ).get_matches();

        let file_name = String::from(args.value_of("file_name").unwrap_or(""));
        let template_name = args.value_of("template_file").unwrap_or("");

        let extension_list: Vec<&str> = file_name.split('.').collect();
        
        ProgramArguments {
            file_name: file_name.clone(),
            template_file: String::from(template_name),
            
            extension: String::from(*extension_list.last().unwrap_or(&"")),
            extension_list: extension_list.iter().map(|str_as_string| String::from(*str_as_string)).collect(),
            file_name_without_extension: String::from(*extension_list.first().unwrap_or(&"")),

            use_explicit_template_file:        args.is_present("template_file"),
            create_matching_header_and_source: args.is_present("matching_headers"),

            create_one_per_platform:    args.is_present("platform"),
            create_one_per_enumeration: args.is_present("enumeration"),
            create_one_per_language:    args.is_present("language"),
            
            overwrite:            args.is_present("overwrite"),
            verbose_output:       args.is_present("verbose"),

            write_file_to_screen: args.is_present("debug"),
            write_names_of_files_to_screen: args.is_present("list_names"),

            show_documentation: args.is_present("show_documentation")
        }
    } 
}