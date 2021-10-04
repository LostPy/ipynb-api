use pyo3::prelude::*;
use pyo3::{PyObject};
use serde_json::json;

use crate::{
    Notebook,
    Cell,
    Output,
    Metadata,
};


#[pyclass(name = "Notebook")]
pub struct PyNotebook {
    rust_notebook: Notebook,
}

impl PyNotebook {
    pub fn from_rust(notebook: Notebook) -> Self {
        Self {rust_notebook: notebook}
    }
}

#[pymethods]
impl PyNotebook {

    #[new]
    fn new(path: String) -> Self {
        Self {
            rust_notebook: Notebook::new(path),
        }
    }

    #[getter]
    fn path(&self) -> &String {
        self.rust_notebook.get_path()
    }

    #[getter]
    fn metadata(&self) {} // Not implemented

    #[getter]
    fn nbformat(&self) -> u8 {
        self.rust_notebook.get_nbformat()
    }

    #[getter]
    fn nbformat_minor(&self) -> u8 {
        self.rust_notebook.get_nbformat_minor()
    }

    #[getter]
    fn cells(&self) {} // Not implemented

    fn load(&mut self) {
        self.rust_notebook.load();
    }

    fn save(&mut self) {
        self.rust_notebook.save();
    }

    fn to_markdown(&self) -> String {
        self.rust_notebook.to_markdown()
    }

    fn export_to_markdown(&self, output: String) {
        self.rust_notebook.export_to_markdown(&output)
    }
}


#[pyclass(name = "Cell")]
pub struct PyCell {
    rust_cell: Cell,
}

impl PyCell {
    pub fn from_rust(cell: Cell) -> Self {
        Self {rust_cell: cell}
    }
}

#[pymethods]
impl PyCell {

    #[new]
    fn new(json_cell: &str) -> Self {
        let cell_data: serde_json::Value  = json!(json_cell);

        Self {
            rust_cell: Cell::new(&cell_data),
        }
    }

    #[getter]
    fn cell_type(&self) -> String {
        self.rust_cell.get_type().to_string()
    }

    #[getter]
    fn id(&self) -> &String {
        self.rust_cell.get_id()
    }

    #[getter]
    fn metadata(&self) {} // Not implemented

    #[getter]
    fn source(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.rust_cell.get_source().to_object(py)
    }

    #[getter]
    fn source_as_str(&self) -> String {
        self.rust_cell.get_source_as_string()
    }

    fn get_outputs(&self) {} // Not implemented

    fn to_markdown(&self) -> String {
        self.rust_cell.to_markdown()
    }
}


#[pyclass(name = "Output")]
pub struct PyOutput {
    rust_output: Output,
}

impl PyOutput {
    pub fn from_rust(output: Output) -> Self {
        Self {rust_output: output}
    }
}

#[pymethods]
impl PyOutput {
    #[new]
    fn new(json_output: &str) -> Self {
        let output_data: serde_json::Value = json!(json_output);

        Self {
            rust_output: Output::new(&output_data),
        }
    }

    #[getter]
    fn name(&self) -> &String {
        self.rust_output.get_name()
    }

    #[getter]
    fn output_type(&self) -> String {
        self.rust_output.get_type().to_string()
    }

    #[getter]
    fn text(&self) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.rust_output.get_text().to_object(py)
    }

    #[getter]
    fn text_as_str(&self) -> String {
        self.rust_output.get_text_as_string()
    }

    #[getter]
    fn evalue(&self) -> String {
        match self.rust_output.get_evalue() {
            Some(ref evalue) => evalue.to_string(),
            None => "".to_string(),
        }
    }

    #[getter]
    fn is_error(&self) -> bool {
        self.rust_output.is_error()
    }

    fn get_traceback(&self) -> String {
        match self.rust_output.get_traceback() {
            Some(ref traceback) => traceback.to_string(),
            None => "".to_string(),
        }
    }

    fn to_text(&self) -> String {
        self.rust_output.to_text()
    }

    fn to_markdown(&self) -> String {
        self.rust_output.to_markdown()
    }
}


#[pyclass(name = "Metadata")]
pub struct PyMetadata {
    rust_metadata: Metadata,
}
