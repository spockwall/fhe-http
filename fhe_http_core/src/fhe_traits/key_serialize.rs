use tfhe::{ClientKey, CompressedCompactPublicKey, CompressedServerKey, ServerKey};

pub trait KeySerialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &Vec<u8>) -> Self;
}

impl KeySerialize for ClientKey {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(data: &Vec<u8>) -> Self {
        bincode::deserialize(data).unwrap()
    }
}

impl KeySerialize for ServerKey {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(data: &Vec<u8>) -> Self {
        bincode::deserialize(data).unwrap()
    }
}

impl KeySerialize for CompressedCompactPublicKey {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(data: &Vec<u8>) -> Self {
        bincode::deserialize(data).unwrap()
    }
}

impl KeySerialize for CompressedServerKey {
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deserialize(data: &Vec<u8>) -> Self {
        bincode::deserialize(data).unwrap()
    }
}
