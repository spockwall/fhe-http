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
