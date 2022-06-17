# Known Bugs 
-m flag does not correctly make the matching file. It uses the same template for both. 

# Future Features
- Ability to specify which formatting to use with DIR_NAME_AS_TYPE
- Support for enviornment variables with a \[\]VAR{env}\[\] template variable
- Support for registry key variables (Windows only) \[\]REGISTRY{key}\[\]
- Support for path variables with \[\]PATH\[\] variable and \[\]FOR_EACH_PATH_VAR{ignore list}\[\]
- Support for relative \[\]DIR{how many dirs up the tree to move}\[\]
- {}NUMBER{} variable for the FOR_EACH_FILE_IN_DIR variable that will evaluate to the iteration number
