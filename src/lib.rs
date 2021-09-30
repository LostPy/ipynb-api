
mod notebook;
mod utils;

pub mod error {
    #[derive(Debug)]
    pub enum Error {
        // Errors Types...
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


#[derive(Debug)]
pub enum OutputType {
    Stream,
    Error,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
