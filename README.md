# TemplateCommand (tt)
Command line utility for creating source files from a set of templates. Template files can have various variables defined that get evaluated when creating a new file. 

## <a href="/DOCUMENTATION.md">Documentation</a>
## <a href="/KNOWN_BUGS.md">Known Bugs</a>
## <a href="/ROADMAP.md">Roadmap</a>
## <a href="/CHANGELOG.md">Changelog</a>

# Install Directions
## Windows
## Linux
## Mac

# Editor Plugins
## Sublime Text 4
### Usage
### Install Directions
<a href="/editor_plugins/sublime_plugin/install_instructions.md">Here</a>

## VS Code
### Usage
### Install Directions
<a href="/editor_plugins/vs_code_plugin/install_instructions.md">Here</a>

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