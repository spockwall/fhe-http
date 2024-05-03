use pyo3::prelude::*;
pub mod apis {
    pub mod fhe_ops;
    pub mod fhe_types;
    pub mod key_generator;
}

#[pymodule]
fn fhe_http_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<apis::key_generator::KeyGenerator>()?;
    m.add_class::<apis::fhe_ops::FheOps>()?;
    Ok(())
}
