use crate::configs::typing::{SerialFheInt64, SerialFheUint64};
use crate::fhe_traits::decryptable::Decryptable;
use tfhe::{FheInt64, FheUint64};

pub trait FheValueSerializable {
    fn serialize(&self) -> Vec<u8>
    where
        Self: Decryptable;
    fn deserialize(data: &Vec<u8>) -> Self;
}
macro_rules! impl_fhe_value_serializable {
    ($t:ty, $s:ty) => {
        impl FheValueSerializable for $t {
            fn serialize(&self) -> $s {
                match bincode::serialize(&self) {
                    Ok(value) => return value,
                    Err(e) => panic!("Failed to serialize: {}", e),
                }
            }

            fn deserialize(data: &$s) -> Self {
                match bincode::deserialize(data) {
                    Ok(value) => return value,
                    Err(e) => panic!("Failed to deserialize: {}", e),
                }
            }
        }
    };
}

impl_fhe_value_serializable!(FheInt64, SerialFheInt64);
impl_fhe_value_serializable!(FheUint64, SerialFheUint64);
