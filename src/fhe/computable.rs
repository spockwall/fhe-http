use crate::configs::fhe_types::FheSupportedType;
use crate::configs::json::FheJsonValue;
use crate::fhe::decryptable::Decryptable;
pub trait Computable {
    fn add(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn sub(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn mul(&self, other: &Self, data_type: &FheSupportedType) -> Self;
}

impl Computable for FheJsonValue {
    fn add(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a + &b)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a + &b)
            }
            _ => panic!("Unsupported data type passed for addition"),
        }
    }
    fn sub(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a - &b)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a - &b)
            }
            _ => panic!("Unsupported data type passed for subtraction"),
        }
    }
    fn mul(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a * &b)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a * &b)
            }
            _ => panic!("Unsupported data type passed for multiplication"),
        }
    }
}
