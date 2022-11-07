import sublime
import sublime_plugin

from subprocess import PIPE, Popen

class CreateFromTemplateCommand(sublime_plugin.WindowCommand):
	def run(self, dirs=[], template=""):
		self.template = template
		self.directory = dirs[0]
		self.window.show_input_panel("File name to create:", "", self.on_done, None, self.on_cancel)

	def on_done(self, text): 
		create_file_from_template(self.template, self.directory + get_seperator_slash() + text)

	def on_cancel(self):
		print()

	template = ""
	directory = ""

class RefreshTemplateListCommand(sublime_plugin.WindowCommand): 
	def run(self): 
		create_side_bar_file()

def create_file_from_template(template, file_name): 
	output_file = file_name + template;
	print(output_file)
	proc = subprocess.Popen(['tt', '-f ' + output_file], stdout=PIPE)
	print(proc.communicate()[0].decode('utf-8'))

def create_side_bar_file(): 
	packages_path = get_plugin_dir();
	side_bar_base_file = "" + open(packages_path + "side_bar_base.json").read();
	template_list = clean_up_template_list(create_side_bar_template_list());
	template_json_string = ""
	for template in template_list: 
		template_json_string += create_json_for_sidebar_item(template)
		template_json_string += ", "
	write_sidebar_file(side_bar_base_file.replace('INSERT_INSTALLED_TEMPLATES', template_json_string))

def write_sidebar_file(json_file_contents): 
	file = open(get_plugin_dir() + "Side Bar.sublime-menu", "w")
	file.write(json_file_contents)
	file.close()

def create_side_bar_template_list(): 
	with subprocess.Popen(['tt', '-z'], stdout=PIPE) as tt: 
		template_list_raw = tt.communicate()[0].decode('utf-8')
		template_list = template_list_raw.split(get_line_ending())
		template_list.pop(0)
		template_list.pop(0)
		return template_list
	return [""]

def create_json_for_sidebar_item(item): 
	return "{ \"caption\": \"" + item + "\", \"command\": \"create_from_template\", \"args\": { \"dirs\": [], \"template\": \"" + item + "\" } }"

def clean_up_template_list(list): 
	cleaned_up_list = []
	for item in list: 
		if item == "":
			continue
		cleaned_up_list.append(remove_template_prefix(item))
	return cleaned_up_list

def remove_template_prefix(item):
	return item[len("template"):]

def get_line_ending(): 
	if (sublime.platform() == "windows"):
		return "\r\n"
	else: 
		return "\n"

def get_seperator_slash(): 
	if (sublime.platform() == "windows"):
		return "\\"
	else: 
		return "/"

def get_plugin_dir(): 
	if (sublime.platform() == "windows"):
		return sublime.packages_path() + "\\template_touch\\"
	else: 
		return sublime.packages_path() + "/template_touch/"