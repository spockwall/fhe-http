use crate::configs::typing::{SerialInt64, SerialUint64};
use crate::fhe_traits::encryptable::Encryptable;
use bincode;

pub trait ValueSerializable: Encryptable + Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}
macro_rules! impl_value_serializable {
    ($t:ty, $s:ty) => {
        impl ValueSerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }

            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_value_serializable!(i64, SerialInt64);
impl_value_serializable!(u64, SerialUint64);
