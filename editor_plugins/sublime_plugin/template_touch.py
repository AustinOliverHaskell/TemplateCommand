import sublime
import sublime_plugin

import subprocess

class CreateFromTemplateCommand(sublime_plugin.WindowCommand):
	def run(self, paths=[]):
		print("Creating from template")
		print(paths)
