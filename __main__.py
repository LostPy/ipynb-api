import argparse


DESCRIPTION = "A command line program to export a Jupyter Notebook file."


def create_parser():
	parser = argparse.ArgumentParser(prog="ipynb_api", description=DESCRIPTION)
	parser.add_argument("notebook", nargs="?", type=str,
		help="The path of notebook file to export.")
	parser.add_argument("--output", nargs="?", type=str, required=False,
		help="The path of output file")
	parser.add_argument("--type", choices=["markdown"], default="markdown",
		help="The type of output file.")
	return parser


if __name__ == "__main__":
	import sys
	import os.path
	from ipynb_api import Notebook

	parser = create_parser()
	args = parser.parse_args(sys.argv[1:])
	print(args)

	if os.path.exists(args.notebook)\
	 and os.path.splitext(args.notebook)[1] == '.ipynb':
		notebook = Notebook(args.notebook)
		output_name = os.path.basename(os.path.splitext(notebook.path)[0])
		if args.type == 'markdown':
			output = output_name + '.md' if not args.output else args.output
			notebook.export_to_markdown(output)
			print(f"The notebook '{args.notebook}' was exported to markdown '{output}'")
	else:
		print(f"The path '{args.notebook}' wasn't found or don't exists, or this is not a .ipynb file")
		sys.exit(1)
	sys.exit(0)