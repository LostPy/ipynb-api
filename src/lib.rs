
mod notebook;
pub mod utils;

pub mod error {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub enum IPyNbError {
        BadJSONValue, // see for an other name
    }

    impl fmt::Display for IPyNbError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                IPyNbError::BadJSONValue => write!(f, "BadJSONValue: {}", "serde_json::Value is not in the asked format."),
            }
        }
    }

    impl Error for IPyNbError {
        fn description(&self) -> &str {
            match *self {
                IPyNbError::BadJSONValue => {return "serde_json::Value is not in the asked format.";},
            }
        }
    }
}


#[derive(Debug)]
pub enum ExportFormat {
    Markdown,
}


#[derive(Debug)]
pub enum CellType {
    Markdown,
    Code,
}

impl CellType {
    pub fn from_str(cell_type : &str) -> Self {
        match cell_type {
            "markdown" => CellType::Markdown,
            "code" => CellType::Code,

            _ => {panic!("{}", "'cell_type' can be 'markdown' or 'code'");}
        }
    }
}


#[derive(Debug)]
pub enum OutputType {
    Stream,
    ExecuteResult,
    Error,
}

impl OutputType {
    pub fn from_str(output_type : &str) -> Self {
        match output_type {
            "stream" => OutputType::Stream,
            "execute_result" => OutputType::ExecuteResult,
            "error" => OutputType::Error,
            _ => panic!("{}", "'output_type' can be 'stream', 'execute_result' or 'error'"),
        }
    }
}

pub use notebook::{
    Notebook,
    Cell,
    Output,
    Metadata,
};

#[cfg(test)]
mod tests {
    use crate::{
        Notebook,
        //Cell,
        //Output,
        CellType,
        OutputType,
        ExportFormat,
    };

    #[test]
    fn test_cell_type_from_str() {
        let cell_code = "code";
        let cell_markdown = "markdown";
        match CellType::from_str(cell_code) {
            CellType::Code => (),
            _ => panic!("bad conversion &str to CellType"),
        };
        match CellType::from_str(cell_markdown) {
            CellType::Markdown => (),
            _ => panic!("bad conversion &str to CellType"),
        };
    }

    #[test]
    fn test_output_type_from_str() {
        match OutputType::from_str("stream") {
            OutputType::Stream => (),
            _ => panic!("bad conversion &str to OutputType"),
        };
        match OutputType::from_str("execute_result") {
            OutputType::ExecuteResult => (),
            _ => panic!("bad conversion &str to OutputType"),
        };
        match OutputType::from_str("error") {
            OutputType::Error => (),
            _ => panic!("bad conversion &str to OutputType"),
        };
    }

    #[test]
    fn test_read_notebook() {
        let nb = Notebook::new("./test.ipynb".to_string());
        println!("{:?}", nb);
        assert_eq!(nb.get_cells().len(), 7);
    }

    #[test]
    fn test_export_markdown() {
        let nb = Notebook::new("./test.ipynb".to_string());
        nb.export(&"export_test.md".to_string(), ExportFormat::Markdown)
    }
}
