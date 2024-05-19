use crate::configs::typing::{SerialInt64, SerialUint64};
use crate::fhe_traits::encryptable::Encryptable;
use bincode;

pub trait ValueSerializable {
    fn serialize(&self) -> Vec<u8>
    where
        Self: Encryptable;
    fn deserialize(data: &Vec<u8>) -> Self;
}
macro_rules! impl_value_serializable {
    ($t:ty, $s:ty) => {
        impl ValueSerializable for $t {
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

impl_value_serializable!(i64, SerialInt64);
impl_value_serializable!(u64, SerialUint64);
