use pyo3::prelude::*;
pub mod apis {
    pub mod fhe;
    pub mod fhe_ops;
    pub mod fhe_types;
    pub mod http;
    pub mod key_generator;
}

#[pymodule]
fn fhe_http_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<apis::key_generator::KeyGenerator>()?;
    m.add_class::<apis::fhe_ops::FheOps>()?;
    m.add_class::<apis::fhe_types::FheTypes>()?;
    m.add_class::<apis::fhe::Fhe>()?;
    m.add_function(wrap_pyfunction!(apis::http::create_fhe_header, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::encrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::decrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::set_server_key_in_body, m)?)?;
    Ok(())
}
