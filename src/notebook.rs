
use crate::{
	ExportFormat,
	CellType,
	OutputType,
};


#[derive(Debug)]
pub struct Notebook {
	path : String,
	metadata : Metadata,
	nbformat : u8,
	nbformat_mirror : u8,
	cells : Vec<Cell>,
}

impl Notebook {
	pub fn new(nb_path : String) -> Self {
		// Read a jupyter notebook file and return a Notebook instance
		unimplemented!()
	}

	pub fn load(&self) {
		// Load the jupyter notebook file
		unimplemented!() 
	}

	pub fn save(&self) {
		// Save the jupyter notebook file
		unimplemented!()
	}

	pub fn export(&self, output : String, export_format : ExportFormat) {
		// Export Notebook in the format specified
		unimplemented!()
	}

	pub fn export_to_markdown(&self, output : String) {
		// Export Notebook in a Markdown file
		unimplemented!()
	}
}


#[derive(Debug)]
pub struct Cell {
	_type : CellType,
	id : usize,
	metadata : Metadata,
	source : Vec<String>,
	outputs : Vec<Output>,
	execution_count : usize,
}

impl Cell {
	pub fn new() -> Self {
		unimplemented!()
	}
}


#[derive(Debug)]
struct Output {
	name : String,
	_type : OutputType,
	text : Vec<String>,
	error : bool,
	error_value : Option<String>,
}

impl Output {
	pub fn new() -> Self {
		unimplemented!()
	}
}


#[derive(Debug)]
struct Metadata {

}

impl Metadata {
	pub fn new() -> Self {
		unimplemented!()
	}
}