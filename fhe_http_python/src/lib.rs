use crate::configs::typing::PyFheValue;
use pyo3::prelude::*;
pub mod apis {
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
    pub mod key_generator;
    pub mod serializer;
}
pub mod utils {
    pub mod conversion;
}

pub mod configs {
    pub mod params;
    pub mod typing;
}

#[pymodule]
fn fhe_http_python(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<apis::key_generator::KeyGenerator>()?;
    m.add_class::<apis::fhe_ops::FheOps>()?;
    m.add_class::<apis::fhe::Fhe>()?;
    m.add_class::<apis::serializer::Serializer>()?;
    m.add_function(wrap_pyfunction!(apis::http::create_fhe_header, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::encrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::decrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::set_server_key_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::get_fhe_value_from_json, m)?)?;
    m.add_function(wrap_pyfunction!(apis::base64::encode_fhe_value, m)?)?;
    m.add_function(wrap_pyfunction!(apis::base64::decode_fhe_value, m)?)?;
    m.add_function(wrap_pyfunction!(configs::typing::create_fhe_value_type, m)?)?;
    m.add_function(wrap_pyfunction!(configs::params::get_pbs_params, m)?)?;
    // Using wrap_enum! to expose the enum
    let fhe_value_enum = py.get_type_bound::<PyFheValue>();
    m.add("FheValue", fhe_value_enum)?;

    Ok(())
}
