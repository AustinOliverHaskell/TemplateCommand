# TemplateCommand (tt)
Command line utility for creating source files from a set of templates. Template files can have various variables defined that get evaluated when creating a new file. 

# Flags

>-b, --blank

Uses -f and creates the file but does not fill it with data. Equivalent to the touch command on linux. 

>-d, --debug 

Prints what will be contained in the template rather than saving it to the file system. 

>-e, --enumeration

Creates one file with each value in the enumeration file appended onto it. 

>--help

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

>-z, --show_templates

Shows all templates that are available to use, also lists the directory in which tt is looking. 

# Options

>-f, --file

File name to create. The extension on the filename determines which template file to use. 

>-h, --harvest

The directory to use for all file name harvesting. See FOR_EACH_FILE_IN_DIR and EACH_FILE_IN_DIR in the template variable section below. 

>-t, --template

Specifies a template file to use.

# Configuration Files

tt uses a singular configuration file that is located alongside the executable. This file will be generated if there isnt one whenever tt is first called. This configuration file contains three lists used for enumerations (-p, -l, and -e command flags) as well as a user defined hashmap that contains user defined variables. (See USER_VAR section down below for more information). 

Example configuration file. (This is the one you'll get by default)
```JSON
{
	"enumeration_list":["c","b","f"],
	"language_list":["en","fr"],
	"platform_list":["windows","linux","mac_os"],
	"user_variables":{
		"LOOPBACK_ADDR":"127.0.0.1",
		"VERSION_MANAGEMENT":"git"
	}
}
```

</br>

# Template Files
Template files are files with the name "tt" and an extension. For example tt.cpp will be used when creating a cpp file. These files are located in a directory called templates alongside the tt executable. 

If you've cloned tt from source, there exists a template_exmaples directory to get you started.

<br />

# Template Variables
tt supports multiple variables that can be added to your template file. These variables will be replaced with various values as defined below. All template variables start and end with "[]". Some of the more complicated template variables also contain a "{}" section. This allows you to supply format specifiers to those variables.  

>\[\]FILE_NAME_AS_TYPE\[\]

Uses whatever is passed to the -f flag without the extension formatted into pascal case. 

>\[\]FILE_NAME_AS_TYPE{-suffix, +suffix}\[\]

Uses whatever is passed to the -f flag without the extension formatted into pascal case. Additionally this will add or subtract whatever is passed in the {}

``` C++ 
[]FILE_NAME_AS_TYPE{-Accessor}[] * []FILE_NAME_AS_TYPE[]::get_model_instance() { /* impl */ }
```

When the above is run with a file name of model_accessor.h the line above will evaluate to

``` C++
Model * ModelAccessor::get_model_instance() { /* impl */ }
```

>\[\]FILE_NAME_AS_TYPE{case}\[\]

Uses whatever is passed to the -f flag without the extension formatted the case specified in the {}.

``` C++ 
[]FILE_NAME_AS_TYPE{camel}[]
[]FILE_NAME_AS_TYPE{pascal}[]
[]FILE_NAME_AS_TYPE{spaced}[]
[]FILE_NAME_AS_TYPE{kabob}[]
[]FILE_NAME_AS_TYPE{lower}[]
[]FILE_NAME_AS_TYPE{upper}[]
```

When the above is run with a file name of my_file_test.h the line above will evaluate to

``` C++
myFileCase
MyFileCase
my file test
my-file-test
my_file_test
MY_FILE_TEST
```

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

>\[\]USER_VAR{Variable name}\[\]

Attempts to lookup a variable with the same name as is inside the brackets. These variables are defined inside the config file under user_variables. 

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


>\[\]EACH_FILE_IN_DIR{ignore list}\[\]

Places the name of every file in the current directory (Or the harvest directory if the -h flag is used). This will add a new line after each file name. Additionally, a comma seperated list of extensions to ignore can be added inside the {} curly brackets. If -h is specified this will contain the path to that file aswell. Without -h this will just use the file name without the path to that file. 

For example, having \[\]EACH_FILE_IN_DIR{h, cpp}\[\] will expand to every file in the current directory but will ignore files with either .h or .cpp extensions. 

>\[\]FOR_EACH_FILE_IN_DIR{ignore list ||| line {}VAR{} line }\[\]

This is the most complex variable that is currently supported. This variable takes two arguments seperated with a tripple pipe |||. The first argument is the set of files to ignore when harvesting files. The second argument is the line that will be repeated for each file. The line provided as the second argument supports the following variables: 

- FILE_NAME
- FILE_NAME_AS_TYPE
- FILE_NAME_WITHOUT_EXTENSION
- FILE_NAME_IN_CAPS
- EXTENSION

The functionality of the above variables is the same as their square-bracket counterparts. 

In addition to the above variables the following are supported:

- PATH - The full path to the harvested file. 

Note: The sub-variable used inside the line uses curly brackets {} rather than square brackets. This was to make the parsing easier. 

>\[\]VERSION\[\]

Evaluates to the current version number of this tool in X.X.X formatting. 

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

<h3>Example file with Harvesting</h3>
A powerfull feature of tt is it's capability to repeat a line for every file in the directory. For example, if you're working with Qt's QML engine, you've most likely come across managing qmldir files. tt can help you create these files with ease. 
<br /><br />

```
// Generated on []CURRENT_DATE[] - []CURRENT_TIME[] by []USER[] with tt v[]VERSION[]

module []DIR[]
[]FOR_EACH_FILE_IN_DIR{qmldir, qrc, pro, c, cpp, h|||{}FILE_NAME_WITHOUT_EXTENSION{} 1.0 {}FILE_NAME{}}[]
designersupported
```

The above template will look in the current directory (or whatever is supplied to the -h option) and repeat {}FILE_NAME_WITHOUT_EXTENSION{} 1.0 {}FILE_NAME{} for each file with an extension not in the ignore list (qmldir, qrc, pro, c, cpp, h) and will evaluate any variables within the curly brackets. 

Output of above example when run on a directory with two qmlfiles

```
// Generated on 12-22-2021 13:29 by austinhaskell with tt v1.1.0

module Example
Button01 1.0 Button01.qml
Button02 1.0 Button02.qml
designersupported
```