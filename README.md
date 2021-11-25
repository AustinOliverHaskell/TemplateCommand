# TemplateCommand (tt)
Command line utility for creating source files from a set of templates. Template files can have various variables defined that get evaluated when creating a new file. 

# Flags

>-b, --blank

Uses -f and creates the file but does not fill it with data. Equivalent to the touch command on linux. 

>-d, --debug 

Prints what will be contained in the template rather than saving it to the file system. 

>-e, --enumeration

Creates one file with each value in the enumeration file appended onto it. 

>-h, --help

Displays a help dialog with this information. 

>-l, --language

Creates one file with each value in the language enumeration file appended onto it. 

>-n, --names

Will print the names of the output files without showing their content or writing the files to disk. Useful if you're doing a lot with the enumerations and you want to see what the output will be. 

>-m, --matching_file

If applicable, creates a matching file for whatever extension is used. A .cpp file will generate a corresponding .h file

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

# Options

>-f, --file

File name to create. The extension on the filename determines which template file to use. 

>-t, --template

Specifies a template file to use.

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

>\[\]FILE_NAME_IN_CAPS\[\]

Evaluates to the file name but in all caps, useful for creating define guards or enums.

>\[\]EXTENSION\[\]

Evaluates to the right most extension of whatever is passed to -f. If there is no extension, this evaluates to the same thing as \[\]FILE_NAME\[\]

>\[\]DIR\[\]

Evaluates to the name of the current directory from which the file will be generated to.

>\[\]DIR_AS_TYPE\[\]

Evaluates to the name of the current directory from which the file will be generated to but as a Pascal case type name. 

>\[\]PWD\[\]

Evaluates to the current path that the file will be generated to. 

>\[\]PARTNER_FILE\[\]

Only used currently when either a c, cpp, h file is created. Generates the opposite file type. 

For example attempting to generate my_file.cpp will evaluate this to be my_file.h 

Note: This variable will be skipped if -m is not present. 

>\[\]CURRENT_DATE\[\]

Evaluates to the current date in dd-mm-yyyy formatting. 

>\[\]CURRENT_TIME\[\]

Evaluates to the current date in hh:mm formatting. 

>\[\]PLATFORM\[\]

Evaluates to the platform name taken from the platform enumeration file. 

Note: This variable will be skipped without the -p flag. 

>\[\]LANGUAGE\[\]

Evaluates to the language name taken from the language enumeration file. 

Note: This variable will be skipped without the -l flag. 

>\[\]ENUMERATION\[\]

Evaluates to the user defined enumeration name taken from the user defined  enumeration file. 

Note: This variable will be skipped without the -e flag. 

>\[\]USER\[\]

Evaluates to the name of the currently logged in user.

>\[\]OS\[\]

Evaluates to the name of the currently running OS.

>\[\]DEVICE_NAME\[\]

Evaluates to the friendly name of the device. This is the same as what shows when pairing the device via bluetooth. 

>\[\]CURRENT_DATE{format}\[\]

Evaluates to the current date but uses whatever is passed to {format} as the format string. 

Example
```C++
#pragma once

// Created by []USER[] on []CURRENT_DATE{%d-%B-%Y}[]

class []FILE_NAME_AS_TYPE[] {
    public:
        []FILE_NAME_AS_TYPE[]()  {};
        ~[]FILE_NAME_AS_TYPE[]() {};
    private:
};
```
creates

```C++
#pragma once

// Created by austinhaskell on 25-November-2021

class MyFile {
    public:
        MyFile()  {};
        ~MyFile() {};
    private:
};

```

Note: This follows the chrono formatting strings. See here: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html

>\[\]CURRENT_TIME{format}\[\]

Evaluates to the current time but uses the format specifier to format the string. For an example see \[\]CURRENT_DATE{format}\[\] above. 

Note: This follows the chrono formatting strings. See here: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html


# Example Templates
<h3>Example C++ Template File</h3>
<br />

.cpp file 

```C++
// Created on []CURRENT_DATE[] by []USER[]

#pragma once

#include "[]PARTNER_FILE[]"

[]FILE_NAME_AS_TYPE[]::[FILE_NAME_AS_TYPE]() {

}

[]FILE_NAME_AS_TYPE[]::~[]FILE_NAME_AS_TYPE[]() {

}
```

.h file 

```C++
#pragma once

class []FILE_NAME_AS_TYPE[] {
    public:
        []FILE_NAME_AS_TYPE[]()  {};
        ~[]FILE_NAME_AS_TYPE[]() {};
    private:
};
```
</br>
</br>
<h3>Example C++ with design pattern</h3>
</br>
Because of the way this tool evaluates extensions, you can create ready to use design patterns. For example, you could create a singleton implementation and then create it in a pinch. 
</br>
</br>

tt.singleton.h
```C++
#pragma once

// Created by []USER[] on []CURRENT_DATE[] []CURRENT_TIME[]

class []FILE_NAME_AS_TYPE[] {
	public:
		static []FILE_NAME_AS_TYPE[] * instance() {
			if (_instance == nullptr) {
				_instance = new []FILE_NAME_AS_TYPE[]();
			}

			return _instance;
		}
	private:
		[]FILE_NAME_AS_TYPE[]() {
		}

		static []FILE_NAME_AS_TYPE[] * _instance;
};
```

So by calling tt with a file name of my_file.singleton.h you'll get a file named my_file.h with the following contents. 

```C++
#pragma once

// Created by austinhaskell on 11-24-2021 15:24

class MyFile {
	public:
		static MyFile * instance() {
			if (_instance == nullptr) {
				_instance = new MyFile();
			}

			return _instance;
		}
	private:
		MyFile() {
		}

		static MyFile * _instance;
};
```