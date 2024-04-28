use fhe_http_core::configs::fhe_types::FheSupportedType;
use pyo3::prelude::*;

#[pyclass]
struct FheSupportedTypeEnumWrapper {
    inner: FheSupportedType,
}
#[pymethods]
impl FheSupportedTypeEnumWrapper {
    #[new]
    fn new(value: String) -> Self {
        let inner = match value.as_str() {
            "Int64" => FheSupportedType::Int64,
            "Uint64" => FheSupportedType::Uint64,
            "String" => FheSupportedType::String,
            _ => panic!("Unsupported type"),
        };
        FheSupportedTypeEnumWrapper { inner }
    }

    fn get_type(&self) -> String {
        match self.inner {
            FheSupportedType::Int64 => "Int64".to_string(),
            FheSupportedType::Uint64 => "Uint64".to_string(),
            FheSupportedType::String => "String".to_string(),
        }
    }
}
