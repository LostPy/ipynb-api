
mod notebook;
mod utils;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
