use tfhe::{ClientKey, ServerKey};

pub trait KeySerialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self;
}

impl KeySerialize for ClientKey {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}

impl KeySerialize for ServerKey {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(data: &[u8]) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
