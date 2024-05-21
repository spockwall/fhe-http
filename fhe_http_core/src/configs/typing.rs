// key_serializable.rs
pub type SerialServerKey = Vec<u8>;
pub type SerialClientKey = Vec<u8>;
pub type SerialCompressedServerKey = Vec<u8>;
pub type SerialCompressedCompactPublicKey = Vec<u8>;

// value_serializable.rs
pub type SerialInt64 = Vec<u8>;
pub type SerialUint64 = Vec<u8>;
//pub type SerialString = Vec<u8>;

// fhe_value_serializable.rs
pub type SerialFheInt64 = Vec<u8>;
pub type SerialFheUint64 = Vec<u8>;
//pub type SerialFheString = Vec<u8>;

// other
pub type StringfiedJson = str;
pub type CompuationResult<T> = Result<T, &'static str>;

#[derive(Debug, Clone)]
pub enum FheValue {
    Int64,
    Uint64,
}

impl FheValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            FheValue::Int64 => "Int64",
            FheValue::Uint64 => "Uint64",
        }
    }

    pub fn from_str(s: &str) -> FheValue {
        match s {
            "Int64" => FheValue::Int64,
            "Uint64" => FheValue::Uint64,
            _ => panic!("Invalid FheValue"),
        }
    }
}
