use tfhe::{FheInt64, FheUint64, FheUint8};

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
