use crate::configs::typing::{SerialPbsParams, SerialPublicZkParams};
use tfhe::shortint::ClassicPBSParameters;
use tfhe::zk::CompactPkePublicParams;
pub trait ZkSerializable: Sized {
    fn try_serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn try_deserialize(data: &Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>;
}
macro_rules! impl_zk_serializable {
    ($t:ty, $s:ty) => {
        impl ZkSerializable for $t {
            fn try_serialize(&self) -> Result<$s, Box<dyn std::error::Error>> {
                bincode::serialize(&self).map_err(|e| e.into())
            }
            fn try_deserialize(data: &$s) -> Result<$t, Box<dyn std::error::Error>> {
                bincode::deserialize(data).map_err(|e| e.into())
            }
        }
    };
}

impl_zk_serializable!(ClassicPBSParameters, SerialPbsParams);
impl_zk_serializable!(CompactPkePublicParams, SerialPublicZkParams);
