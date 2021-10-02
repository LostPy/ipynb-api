use std::fs;
use std::io::Write;
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
    nbformat_minor : u8,
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
            nbformat_minor: json_nb["nbformat_minor"].as_u64().unwrap() as u8,
            cells: Cell::new_vec(&json_nb["cells"]),
        }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn get_nbformat(&self) -> u8 {
        self.nbformat
    }

    pub fn get_nbformat_mirror(&self) -> u8 {
        self.nbformat_minor
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn load(&mut self) {
        // Load the jupyter notebook file
        // This function panic if the value of json is not the expected value
        let data = fs::read_to_string(&self.path).unwrap();
        let json_nb: json::Value = json::from_str(&data).expect("The JSON file was not well-formatted");

        self.metadata = Metadata::new();
        self.nbformat = json_nb["nbformat"].as_u64().unwrap() as u8;
        self.nbformat_minor = json_nb["nbformat_minor"].as_u64().unwrap() as u8;
        self.cells = Cell::new_vec(&json_nb["cells"]);
    }

    pub fn save(&self) {
        // Save the jupyter notebook file
        unimplemented!()
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        for cell in &self.cells {
            markdown.push_str(&format!("{}\n", cell.to_markdown()));
        }
        markdown
    }

    pub fn export(&self, output : &String, export_format : ExportFormat) {
        // Export Notebook in the format specified
        match export_format {
            ExportFormat::Markdown => self.export_to_markdown(output),
        }
    }

    pub fn export_to_markdown(&self, output : &String) {
        // Export Notebook in a Markdown file
        let mut file = fs::File::create(output).unwrap();
        file.write_all(self.to_markdown().as_bytes()).unwrap();
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
        let cell_type = CellType::from_str(&json_cell["cell_type"].as_str().unwrap());

        Self {
            cell_type: CellType::from_str(json_cell["cell_type"].as_str().unwrap()),
            id: json_cell["id"].as_str().unwrap().to_string(),
            metadata: Metadata::new(),
            source: json::from_str(&json_cell["source"].to_string()).unwrap(),
            outputs: match cell_type {
                CellType::Markdown => Option::None,
                _ => Option::Some(Output::new_vec(&json_cell["outputs"])),
            },
            execution_count: if json_cell["execution_count"].is_null() {
                Option::None
            } else {
                Option::Some(json::from_str(&json_cell["execution_count"].to_string()).unwrap())
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

    pub fn get_type(&self) -> &CellType {
        &self.cell_type
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn get_source(&self) -> &Vec<String> {
        &self.source
    }

    pub fn get_source_as_string(&self) -> String {
        utils::vec_to_string(&self.source)
    }

    pub fn get_outputs(&self) -> &Option<Vec<Output>> {
        &self.outputs
    }

    pub fn get_execution_count(&self) -> &Option<usize> {
        &self.execution_count
    }

    pub fn outputs_to_markdown(&self) -> String {
        let mut markdown: String = "Output\n```\n".to_string();

        match self.outputs {
            Some(ref outputs) => for output in outputs {
                markdown.push_str(&format!("{}\n", output.to_text()));
            },
            None => {return String::new();},
        }
        markdown = markdown.trim().to_string();
        markdown.push_str("\n```");
        markdown
    }

    pub fn to_markdown(&self) -> String {
        let cell_md = match self.cell_type {
            CellType::Markdown => self.get_source_as_string(),
            _ => format!("```\n{}\n```", self.get_source_as_string()),
        };

        format!("{}\n{}\n", cell_md, self.outputs_to_markdown())
    }
}



#[derive(Debug)]
pub struct Output {
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
                output_type: OutputType::from_str(&result["output_type"].as_str().unwrap()),
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

    pub fn to_text(&self) -> String {
        // Return a String with the text of output, if self is an error: the first line is the error_value
        match self.error_value {
            Some(ref evalue) => format!("{}\n{}", evalue, self.get_text_as_string()),
            None => self.get_text_as_string(),
        }
    }

    pub fn to_markdown(&self) -> String {
        // Return the result of `self.to_text` in a Markdown code block
        format!("```\n{}```", self.to_text())
    }
}


#[derive(Debug)]
pub struct Metadata {

}

impl Metadata {
    pub fn new() -> Self {
        Self {}
    }
}