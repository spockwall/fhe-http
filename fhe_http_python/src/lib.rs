use crate::configs::typing::{
    create_fhe_value_type, create_proven_fhe_value_type, PyFheValue, PyProvenFheValue,
};
use pyo3::prelude::*;
pub mod apis {
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
    pub mod key_generator;
    pub mod proven_fhe_ops;
    pub mod serializer;
    pub mod server_key_setter;
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
    m.add_class::<apis::fhe::Fhe>()?;
    m.add_class::<apis::fhe_ops::FheOps>()?;
    m.add_class::<apis::key_generator::KeyGenerator>()?;
    m.add_class::<apis::proven_fhe_ops::ProvenFheOps>()?;
    m.add_class::<apis::serializer::Serializer>()?;
    m.add_class::<apis::server_key_setter::ServerKeySetter>()?;
    m.add_function(wrap_pyfunction!(apis::base64::encode_fhe_value, m)?)?;
    m.add_function(wrap_pyfunction!(apis::base64::decode_fhe_value, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::create_fhe_header, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::encrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::decrypt_fhe_body, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::set_server_key_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(apis::http::get_fhe_value_from_json, m)?)?;
    m.add_function(wrap_pyfunction!(configs::params::get_public_zk_params, m)?)?;
    m.add_function(wrap_pyfunction!(create_fhe_value_type, m)?)?;
    m.add_function(wrap_pyfunction!(create_proven_fhe_value_type, m)?)?;

    // Using wrap_enum! to expose the enum
    let fhe_value_enum = py.get_type_bound::<PyFheValue>();
    let proven_fhe_value_enum = py.get_type_bound::<PyProvenFheValue>();
    m.add("FheValue", fhe_value_enum)?;
    m.add("ProvenFheValue", proven_fhe_value_enum)?;
    Ok(())
}
