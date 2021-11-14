pub fn print_all_variables() {
    let variable_name_list = vec![
        "FILE_NAME_AS_TYPE", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        "", 
        ""
        ];

    for variable in variable_name_list {
        print_variable(variable);
    }
}

pub fn print_variable(var_name: &str) {
    match var_name {
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME" => {
            println!("FILE_NAME -> Uses the file name and extension of the output file")
        },
        "EXTENSION" => {
            println!("EXTENSION -> Evaluates to the right most extension of whatever is passed to -f. If there is no extension, this \
            evaluates to the same thing as FILE_NAME")
        },
        "PARENT_DIR" => {
            println!("PARENT_DIR -> Evaluates to the name of the parent directory from which the file will be generated to.")
        },
        "PARENT_DIR{}" => {
            println!("PARENT_DIR{{}} -> Evaluates to the name of the parent directory from which the file will be generated to. \
            This command will traverse up the directory tee an integer number up steps according to what is in the {{}}")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        "FILE_NAME_AS_TYPE" => {
            println!("FILE_NAME_AS_TYPE -> Uses whatever is passed to the -f flag without the extension, formatted to Pascal case")
        },
        _ => {}
    }
}



/*
>\[\]PARENT_DIR{}\[\]

Evaluates to the name of the parent directory from which the file will be generated to. This command will traverse up the directory tee an integer number up steps according to what is in the "{}"

>\[\]PARENT_DIR_AS_TYPE\[\]

Evaluates to the name of the parent directory from which the file will be generated to but following whatever type formatting you have specified. 

>\[\]PARTNER_FILE\[\]

Only used currently when either a c, cpp, h file is created. Generates the opposite file type. 

For example attempting to generate my_file.cpp will evaluate this to be my_file.h 

Note: This variable will be skipped if -m is not present. 

>\[\]CURRENT_DATE\[\]

Evaluates to the current date in dd-mm-yyyy formatting. 

>\[\]CURRENT_DATE{}\[\]

Evaluates to the current date. Uses whatever is between the "{}" as the format string.  

>\[\]CURRENT_TIME\[\]

Evaluates to the current date in hh:mm formatting. 

>\[\]CURRENT_TIME{}\[\]

Evaluates to the current date. Uses whatever is between the "{}" as the format string. 

>\[\]PLATFORM\[\]

Evaluates to the platform name taken from the platform enumeration file. 

Note: This variable will be skipped without the -p flag. 

>\[\]LANGUAGE\[\]

Evaluates to the language name taken from the language enumeration file. 

Note: This variable will be skipped without the -l flag. 

>\[\]ENUMERATION\[\]

Evaluates to the user defined enumeration name taken from the user defined  enumeration file. 

Note: This variable will be skipped without the -e flag. 

>\[\]USER_DEF{}\[\]

Searches the user definition file for a variable named the same as whatever is in the "{}". If no match is found this variable will be skipped. 

>\[\]ENV{}\[\]

Searches the users path for a variable named the same as whatever is in the "{}". If no match is found this variable will be skipped. 

>\[\]USER\[\]

Evaluates to the name of the currently logged on user. 
*/