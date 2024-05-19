use crate::configs::typing::{
    SerialClientKey, SerialCompressedCompactPublicKey, SerialCompressedServerKey, SerialServerKey,
};
use tfhe::{ClientKey, CompressedCompactPublicKey, CompressedServerKey, ServerKey};
pub trait KeySerializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &Vec<u8>) -> Self;
}
macro_rules! impl_key_serializable {
    ($t:ty, $s:ty) => {
        impl KeySerializable for $t {
            fn serialize(&self) -> $s {
                bincode::serialize(&self).unwrap()
            }
            fn deserialize(data: &$s) -> Self {
                bincode::deserialize(data).unwrap()
            }
        }
    };
}

impl_key_serializable!(ClientKey, SerialClientKey);
impl_key_serializable!(ServerKey, SerialServerKey);
impl_key_serializable!(CompressedCompactPublicKey, SerialCompressedCompactPublicKey);
impl_key_serializable!(CompressedServerKey, SerialCompressedServerKey);
