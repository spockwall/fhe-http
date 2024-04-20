use serde_json::Value;
use tfhe::{FheInt64, FheUint64, FheUint8};

pub enum FheJsonValue {
    Value(Value),
    EncInt64(FheInt64),
    EncUint64(FheUint64),
    EncString(Vec<FheUint8>),
}
