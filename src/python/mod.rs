use pyo3::prelude::*;

mod class;


#[pymodule]
fn ipynb_api(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<class::PyNotebook>()?;
    module.add_class::<class::PyCell>()?;
    module.add_class::<class::PyOutput>()?;
    module.add_class::<class::PyMetadata>()?;
    Ok(())
}