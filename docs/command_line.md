
# Options

>-b, --blank [filename]

Uses the file name passed to create an empty file. Equivalent to the touch command on linux. 

>-d, --debug [filename]

Prints what will be contained in the template rather than saving it to the file system. 

>-f, --file [filename]

File name to create. The extension on the filename determines which template file to use. 

>-r, --harvest [path]

The directory to use for all file name harvesting. See FOR_EACH_FILE_IN_DIR and EACH_FILE_IN_DIR in the template variable section below. 

>-t, --template

Specifies a template file to use.

>-h, --header

Uses a found header.x file and prepends it to the specified file. 

# Flags

>-e, --enumeration

Creates one file with each value in the enumeration file appended onto it. 

>--help

Displays a help dialog with this information. 

>-l, --language

Creates one file with each value in the language enumeration file appended onto it. 

>-n, --names

Will print the names of the output files without showing their content or writing the files to disk. Useful if you're doing a lot with the enumerations and you want to see what the output will be. 

>-m, --matching_file

If applicable, creates a matching file for whatever extension is used. This command uses the partner_file_map defined in the configuration to determine which companion file to create. 

>-o, --overwrite

If present, will overwrite any files with the same name as the generated files. 

>-p, --platform

Creates one file with each value in the platform enumeration file appended onto it. 

>-s, --show_doc

Shows all implemented variables and what they evaluate to. 

>-V, --version

Prints the version information. 

>-v, --verbose

Turns on verbose mode. You'll get any debug statements I left in here, so do with that what you will. 

>-z, --show_templates

Shows all templates that are available to use, also lists the directory in which tt is looking. 