use crate::configs::typing::{
    SerialClientKey, SerialCompressedCompactPublicKey, SerialCompressedServerKey, SerialServerKey,
};
use tfhe::{ClientKey, CompressedCompactPublicKey, CompressedServerKey, ServerKey};
pub trait KeySerializable: Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}
macro_rules! impl_key_serializable {
    ($t:ty, $s:ty) => {
        impl KeySerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }
            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_key_serializable!(ClientKey, SerialClientKey);
impl_key_serializable!(ServerKey, SerialServerKey);
impl_key_serializable!(CompressedCompactPublicKey, SerialCompressedCompactPublicKey);
impl_key_serializable!(CompressedServerKey, SerialCompressedServerKey);
