from setuptools import setup

__license__ = "GPL"

setup(
	name="ipynb_api",
	version="0.1.0",
	description="A python api for Jupyter Notebook written in Rust.",
	packages=["ipynb_api"],
	author="LostPy",
	license=__license__,
	zip_safe=False
)