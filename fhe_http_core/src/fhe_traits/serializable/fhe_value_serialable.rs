use crate::configs::typing::{SerialFheInt64, SerialFheUint64};
use crate::fhe_traits::decryptable::Decryptable;
use tfhe::{FheInt64, FheUint64};

pub trait FheValueSerializable: Decryptable + Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}
macro_rules! impl_fhe_value_serializable {
    ($t:ty, $s:ty) => {
        impl FheValueSerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }

            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_fhe_value_serializable!(FheInt64, SerialFheInt64);
impl_fhe_value_serializable!(FheUint64, SerialFheUint64);
