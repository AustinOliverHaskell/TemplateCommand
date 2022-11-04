# ----- Setup -----
if [ -d "run" ] 
then 
	echo "Removing files from last run..."
	rm -r run
fi

mkdir run
cd ..
cd target/debug
mkdir templates
cd ../..
cp -r integration_testing/integration_templates/* target/debug/templates/

cd integration_testing/run
# The templates dir under debug should now have all of the template 
#  files under integration_templates
# ----- End Setup -----

# ----- Run templates -----
cargo run -- -b blank_file_test.cpp

cargo run -- -f define_guards.define_guard_test.h
cargo run -- -f as_type_base_test.file_name_as_type_base_test.rs
cargo run -- -f as_type_formatting_test.file_name_as_type_formatting_test.rs
# -------------------------

# ----- Compare Output -----

# --------------------------