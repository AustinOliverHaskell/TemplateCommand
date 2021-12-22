# Known Bugs 

# Future Features
- Ability to specify which formatting to use for FILE_NAME_AS_TYPE and DIR_NAME_AS_TYPE
- qml and ui.qml as matching file types
- Force File Generation Flag - This would skip any unknown variable errors
- Support for enviornment variables with a \[\]VAR{env}\[\] template variable
- Support for registry key variables (Windows only) \[\]REGISTRY{key}\[\]
- Support for path variables with \[\]PATH\[\] variable and \[\]FOR_EACH_PATH_VAR{ignore list}\[\]
- Matching indentation level for lines created with FOR_EACH_FILE_IN_DIR
- Support for relative \[\]DIR{how many dirs up the tree to move}\[\]
- Support for user defined variables inside template file (This is a big complexity jump and I cant think of many uses for it)
- {}NUMBER{} variable for the FOR_EACH_FILE_IN_DIR variable that will evaluate to the iteration number