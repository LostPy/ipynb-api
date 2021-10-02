use std::error;
use std::fs;
use serde;
use serde_json as json;
use crate::{
    ExportFormat,
    CellType,
    OutputType,
    utils,
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
    pub fn new(path : String) -> Self {
        // Read a jupyter notebook file and return a Notebook instance
        // This function panic if the value of json is not the expected value
        let data = fs::read_to_string(&path).unwrap();
        let json_nb: json::Value = json::from_str(&data).expect("The JSON file was not well-formatted");

        Self {
            path,
            metadata: Metadata::new(),
            nbformat: json_nb["nbformat"].as_u64().unwrap() as u8,
            nbformat_mirror: json_nb["nbformat_mirror"].as_u64().unwrap() as u8,
            cells: Cell::new_vec(&json_nb["cells"]),
        }
    }

    pub fn load(&mut self) {
        // Load the jupyter notebook file
        // This function panic if the value of json is not the expected value
        let data = fs::read_to_string(&self.path).unwrap();
        let json_nb: json::Value = json::from_str(&data).expect("The JSON file was not well-formatted");

        self.metadata = Metadata::new();
        self.nbformat = json_nb["nbformat"].as_u64().unwrap() as u8;
        self.nbformat_mirror = json_nb["nbformat_mirror"].as_u64().unwrap() as u8;
        self.cells = Cell::new_vec(&json_nb["cells"]);
    }

    pub fn save(&self) {
        // Save the jupyter notebook file
        unimplemented!()
    }

    pub fn export(&self, output : &String, export_format : ExportFormat) {
        // Export Notebook in the format specified
        match export_format {
            ExportFormat::Markdown => self.export_to_markdown(output),
        }
    }

    pub fn export_to_markdown(&self, output : &String) {
        // Export Notebook in a Markdown file
        unimplemented!()
    }
}


#[derive(Debug)]
pub struct Cell {
    cell_type : CellType,
    id : String,
    metadata : Metadata,
    source : Vec<String>,
    outputs : Option<Vec<Output>>,
    execution_count : Option<usize>,
}

impl Cell {
    pub fn new(json_cell: &json::Value) -> Self {
        // This function panic if the value of json is not the expected value

        // use to check if cell has an output and execution_cout key:
        let cell_type = CellType::from_str(&json_cell["cell_type"].to_string());

        Self {
            cell_type: CellType::from_str(&json_cell["cell_type"].to_string()),
            id: json_cell["id"].to_string(),
            metadata: Metadata::new(),
            source: json::from_str(&json_cell["source"].to_string()).unwrap(),
            outputs: match cell_type {
                CellType::Markdown => Option::None,
                _ => Option::Some(Output::new_vec(&json_cell["outputs"])),
            },
            execution_count: match cell_type {
                CellType::Markdown => Option::None,
                _ => Option::Some(json::from_str(&json_cell["execution_count"].to_string()).unwrap()),
            },
        }
    }

    pub fn new_vec(json_cells: &json::Value) -> Vec<Self> {
        // This function panic if the value of json is not the expected value
        let mut cells: Vec<Self> = Vec::new();

        for json_cell in json_cells.as_array().unwrap() {
            cells.push(Self::new(json_cell));
        }
        cells
    }
}



#[derive(Debug)]
struct Output {
    name : String,
    output_type : OutputType,
    text : Vec<String>,
    error : bool,
    error_value : Option<String>,
}

impl Output {
    pub fn new(json_output: &json::Value) -> Self {
        // This function panic if the value of json is not the expected value

        let output: Self = match utils::read::normalize_output(json_output) {
            Ok(ref result) => Self {
                name: result["name"].to_string(),
                output_type: OutputType::from_str(&result["output_type"].to_string()),
                text: json::from_str(&result["text"].to_string()).unwrap(),
                error: json::from_str(&result["error"].to_string()).unwrap(),
                error_value: if result["error_value"] == json::Value::Null {
                    Option::None
                } else {
                    Option::Some(result["error_value"].to_string())
                },
            },
            Err(ref e) => panic!("{:?}", e),
        };
        output
    }

    pub fn new_vec(json_outputs: &json::Value) -> Vec<Self> {
        // This function panic if the value of json is not the expected value
        let mut outputs: Vec<Self> = Vec::new();

        for json_output in json_outputs.as_array().unwrap() {
            outputs.push(Self::new(json_output));
        }
        outputs
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_type(&self) -> &OutputType {
        &self.output_type
    }

    pub fn get_text(&self) -> &Vec<String> {
        &self.text
    }

    pub fn get_text_as_string(&self) -> String {
        utils::vec_to_string(&self.text)
    }

    pub fn get_evalue(&self) -> &Option<String> {
        &self.error_value
    }

    pub fn get_traceback(&self) -> Option<String> {
        if self.error {
            return Option::Some(utils::vec_to_string(&self.text));
        }
        Option::None
    }

    pub fn is_error(&self) -> bool {
        self.error
    }
}


#[derive(Debug)]
struct Metadata {

}

impl Metadata {
    pub fn new() -> Self {
        Self {}
    }
}