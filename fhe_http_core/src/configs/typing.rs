use tfhe::{FheInt64, FheUint64, FheUint8};

pub type SerializedServerKey = Vec<u8>;
pub type SerializedClientKey = Vec<u8>;
pub type StringfiedJson = str;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum NorJsonValue {
    Int64(i64),
    Uint64(u64),
    String(String),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum FheJsonValue {
    FheInt64(FheInt64),
    FheUint64(FheUint64),
    FheString(Vec<FheUint8>),
}

pub enum FheSupportedType {
    Int64,
    Uint64,
    String,
}

#[allow(dead_code)]
impl FheSupportedType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FheSupportedType::Int64 => "Int64",
            FheSupportedType::Uint64 => "Uint64",
            FheSupportedType::String => "String",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "Int64" => FheSupportedType::Int64,
            "Uint64" => FheSupportedType::Uint64,
            "String" => FheSupportedType::String,
            _ => panic!("Unsupported type"),
        }
    }
}
