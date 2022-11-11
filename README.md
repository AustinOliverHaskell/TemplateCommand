# TemplateCommand (tt)
Command line utility for creating source files from a set of templates. Template files can have various variables defined that get evaluated when creating a new file. 

## <a href="/DOCUMENTATION.md">Documentation</a>


# Configuration Files

tt uses a singular configuration file that is located alongside the executable. This file will be generated if there isnt one whenever tt is first called. 

This configuration file contains three lists used for enumerations (-p, -l, and -e command flags), a user defined hashmap that contains user defined variables. (See USER_VAR section down below for more information), and another user defined hashmap that contains the mapping for the PARTNER_FILE variable as well as the -m flag. 

Example configuration file. (This is the one you'll get by default)
```JSON
{
	"enumeration_list":["c","b","f"],
	"language_list":["en","fr"],
	"platform_list":["windows","linux","mac_os"],
	"user_variables":{
		"LOOPBACK_ADDR":"127.0.0.1",
		"VERSION_MANAGEMENT":"git"
	}, 
	"partner_file_map":{
		"c":"h",
		"cpp":"h",
		"h":"cpp",
	}
}
```

</br>

# Template Files
Template files are files with the name "template" and an extension. For example template.cpp will be used when creating a cpp file. These files are located in a directory called templates alongside the tt executable. 

If you've cloned tt from source, there exists a template_exmaples directory to get you started.

For the -h option, there also can exist header.[some extension] files in the template directory. These function the exact same as template files but are only used when using -h to append the file to the front of another. 

<br />

# Template Variables
tt supports multiple variables that can be added to your template file. These variables will be replaced with various values as defined below. All template variables start and end with "[]". Template variables can be nested within others, see the FOR_EACH_FILE_IN_DIR variable for more info. Some template variables have required or optional parameters to further configure them.   

<h3>BANNER{Symbol|||Text}</h3>

Uses the Symbol passed to it to create a banner around the text. For example BANNER{*|||THIS IS TEXT} will create the following

``` C++
/*
****************
* THIS IS TEXT * 
****************
*/
```

<h3>[]FILE_NAME_AS_TYPE{-suffix, +suffix, case}[]</h3>

This is the most common variable you'll use. It has several optional parameters that you can specify. 

1) Change case
	camel, pascal, spaced, kabob, lower, upper will format the file name to that case
2) Add ending
	+xxxxxxxx will add whatever x is to the end of the string
3) Remove ending
	-xxxxxxxx will remove whatever x is off the end of the file name, note that this will generate a warning if that ending does not exist. In the case of an ending not existing, the file name unmodified will be placed there instead

Uses whatever is passed to the -f flag without the extension formatted into pascal case by default. Additionally this will add or subtract whatever is passed in the {}

``` C++ 
[]FILE_NAME_AS_TYPE{-Accessor}[] * []FILE_NAME_AS_TYPE[]::get_model_instance() { /* impl */ }
```

When the above is run with a file name of model_accessor.h the line above will evaluate to

``` C++
Model * ModelAccessor::get_model_instance() { /* impl */ }
```

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

An important nuance about this command is that you can chain as many formats and addition/subtractions you want. For example: 

```C++
[]FILE_NAME_AS_TYPE{-model, upper, +Manager}[]
```

Will generate the following when ran with my_test_model.cpp

```C++
MY_TESTManager
```

> Note: Leaving off the {} will format to the default case formatting (Pascal)

<h3>FILE_NAME{-/+ending, case}</h3>

Uses the file name and extension of the output file but subtracts or appends a different ending before the extension. Additionally a case can be specified. See FILE_NAME_AS_TYPE for a more in-depth explanation as well as a list of supported case names

For example
```
FILE_NAME{-_file}
```

ran with a file name of my_file.txt will produce my.txt

<h3>FORCE_FILE_NAME{name}</h3>

Forces the output file name to be "name" this will produce a warning when a template file with this symbol is loaded. 

For example a template file like the following

<h4>other.qmldir</h4>

```c++
[]FORCE_FILE_NAME{qmldir}[]

#File name set to qmldir!!!
```
 will always produce a file called qmldir regardless of what's passed to the -f flag. 

<h3>EXTENSION</h3>

Evaluates to the right most extension of whatever is passed to -f. If there is no extension, this evaluates to the same thing as FILE_NAME

<h3>DIR</h3>

Evaluates to the name of the current directory from which the file will be generated to.

<h3>DIR_AS_TYPE</h3>

Evaluates to the name of the current directory from which the file will be generated to but as a Pascal case type name. 

<h3>PWD</h3>

Evaluates to the current path that the file will be generated to. 

<h3>PARTNER_FILE</h3>

Uses the user defined hashmap in the configuration file to evaluate what is the corresponding file name. 

For example attempting to generate my_file.cpp this will evaluate to be my_file.h 

<h3>PLATFORM</h3>

Evaluates to the platform name taken from the platform enumeration file. 

Note: This variable will be skipped without the -p flag. 

<h3>LANGUAGE</h3>

Evaluates to the language name taken from the language enumeration file. 

Note: This variable will be skipped without the -l flag. 

<h3>ENUMERATION</h3>

Evaluates to the user defined enumeration name taken from the user defined  enumeration file. 

Note: This variable will be skipped without the -e flag. 

<h3>USER</h3>

Evaluates to the name of the currently logged in user.

<h3>USER_VAR{Variable name}</h3>

Attempts to lookup a variable with the same name as is inside the brackets. These variables are defined inside the config file under user_variables. 

<h3>OS</h3>

Evaluates to the name of the currently running OS.

<h3>DEVICE_NAME</h3>

Evaluates to the friendly name of the device. This is the same as what shows when pairing the device via bluetooth. 

<h3>CURRENT_DATE{format}</h3>

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

>Note: This follows the chrono formatting strings. See here: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html

>Note: Defaults to dd-mm-yyyy formatting when no parameter is supplied

<h3>CURRENT_TIME{format}</h3>

Evaluates to the current time but uses the format specifier to format the string. For an example see CURRENT_DATE{format} above. 

>Note: This follows the chrono formatting strings. See here: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html

>Note: Defaults to hh:mm formatting when no parameter is defined. 


<h3>EACH_FILE_IN_DIR{include list}</h3>

Places the name of every file in the current directory (Or the harvest directory if the -h flag is used). This will add a new line after each file name. Additionally, a comma seperated list of extensions to include can be added inside the {} curly brackets. If -h is specified this will contain the path to that file aswell. Without -h this will just use the file name without the path to that file. 

Running without an include list, ie empty brackets, will include all files in that directory. 

For example, having EACH_FILE_IN_DIR{h, cpp} will expand to every file in the current directory but will only use  files with either .h or .cpp extensions. 

<h3>FOR_EACH_FILE_IN_DIR{include list ||| line \[\]VAR\[\] line }</h3>

This is the most complex variable that is currently supported. This variable takes two arguments seperated with a tripple pipe |||. The first argument is the set of files to include when harvesting files. The second argument is the line that will be repeated for each file. 

This variable can have any of the other replacement variables inside it, doing so however changes the meaning of the following variables

- FILE_NAME
- FILE_PATH
- EXTENSION 
- FILE_NAME_WITH_EXTENSION

These will instead evaluate to the file that got harvested. So if using my_file.test as the template, running FOR_EACH_FILE_IN_DIR{h|||[]FILE_NAME[] 1.0 []FILE_NAME_WITH_EXTENSION[]} will generate something similar to below. 

``` txt 
a 1.0 a.h 
b 1.0 b.h 
c 1.0 c.h
```

The line provided as the second argument supports the following additional variables: 

- THIS_FILES_NAME
- THIS_FILES_PATH
- THIS_FILES_EXTENSION

The variable above evaluate to the root file's context. So for example running with my_file.test, THIS_FILES_NAME will be my_file, THIS_FILES_PATH will be ./my_file.test and THIS_FILES_EXTENSION will be test. 


<h3>VERSION</h3>

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
[]FOR_EACH_FILE_IN_DIR{qml, ui.qml|||[]FILE_NAME_WITHOUT_EXTENSION[] 1.0 []FILE_NAME[][]
designersupported
```

The above template will look in the current directory (or whatever is supplied to the -h option) and repeat []FILE_NAME_WITHOUT_EXTENSION[] 1.0 []FILE_NAME[] for each file with an extension in the include list (qml, ui.qml). 

Output of above example when run on a directory with two qml files

```
// Generated on 12-22-2021 13:29 by austinhaskell with tt v1.1.0

module Example
Button01 1.0 Button01.qml
Button02 1.0 Button02.qml
designersupported
```