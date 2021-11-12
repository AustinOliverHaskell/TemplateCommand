<h1>NOTE: MOST OF THIS IS NOT IMPLEMENTED</h1>

# TemplateCommand (tt)
Command line utility for creating source files from a set of templates. 

# Flags

>-f, --file

File name to create. The extension on the filename determines which template file to use. 

>-t, --template

Specifies a template file to use. 

>-m, --matching_file

If applicable, creates a matching file for whatever extension is used. A .cpp file will generate a corresponding .h file

>-p, --platform

Creates one file with each value in the platform enumeration file appended onto it. 

>-l, --language

Creates one file with each value in the language enumeration file appended onto it. 

>-e, --enumeration

Creates one file with each value in the enumeration file appended onto it. 

>-d, --debug 

Prints what will be contained in the template rather than saving it to the file system. 

>-V, --version

Prints the version information. 

>-v, --verbose

Turns on verbose mode. You'll get any debug statements I left in here, so do with that what you will. 

>-h, --help

Displays a help dialog with this information. 

>-o, --overwrite

If present, will overwrite any files with the same name as the generated files. 

# Configuration Files

All of the following files are lists of comma separated values. Running a command that one of these requires without having the file on your system will generate a default one. This file is located alongside tt.exe. 

<h2>Language List</h2>
List of languages to use for the -l flag
<h2>Platform List</h2>
List of platforms to use for the -p flag
<h2>Enumeration List</h2>
List of user defined enumerations to use for the -e flag

</br>
</br>

<h2>Template Files</h2>
Template files are files with the name "tt" and an extension. For example tt.cpp will be used when creating a cpp file. These files are located in a directory called templates alongside the tt executable. 

<br />

# Template Variables
tt supports multiple variables that can be added to your template file. These variables will be replaced with various values as defined below. All template variables start and end with "[]"

>\[\]FILE_NAME_AS_TYPE\[\]

Uses whatever is passed to the -f flag without the extension

>\[\]FILE_NAME\[\]

Uses the file name and extension of the output file

>\[\]EXTENSION\[\]

Evaluates to the right most extension of whatever is passed to -f. If there is no extension, this evaluates to the same thing as \[\]FILE_NAME\[\]

>\[\]PARENT_DIR\[\]

Evaluates to the name of the parent directory from which the file will be generated to.

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

# Example Templates
<h3>Example C Template File</h3>
<h3>Example Rust Template File</h3>