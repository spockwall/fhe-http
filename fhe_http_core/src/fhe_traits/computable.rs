use crate::configs::fhe_types::FheSupportedType;
use crate::configs::json::FheJsonValue;
use crate::fhe_traits::decryptable::Decryptable;
pub trait Computable {
    fn add(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn sub(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn mul(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn div(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn rem(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn and(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn or(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn xor(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn shr(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn shl(&self, other: &Self, data_type: &FheSupportedType) -> Self;
    fn neg(&self, data_type: &FheSupportedType) -> Self;
    fn not(&self, data_type: &FheSupportedType) -> Self;
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
    fn and(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a & (&b))
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a & (&b))
            }
            _ => panic!("Unsupported data type passed for bitwise AND operation"),
        }
    }
    fn div(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a / &b)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a / &b)
            }
            _ => panic!("Unsupported data type passed for division"),
        }
    }
    fn rem(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a % &b)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a % &b)
            }
            _ => panic!("Unsupported data type passed for remainder operation"),
        }
    }
    fn or(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a | (&b))
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a | (&b))
            }
            _ => panic!("Unsupported data type passed for bitwise OR operation"),
        }
    }
    fn xor(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                let b = other.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(a ^ (&b))
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a ^ (&b))
            }
            _ => panic!("Unsupported data type passed for bitwise XOR operation"),
        }
    }
    fn shr(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a >> (&b))
            }
            _ => panic!("Unsupported data type passed for bitwise right shift operation"),
        }
    }
    fn shl(&self, other: &Self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                let b = other.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(a << (&b))
            }
            _ => panic!("Unsupported data type passed for bitwise left shift operation"),
        }
    }
    fn neg(&self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(-a)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(-a)
            }
            _ => panic!("Unsupported data type passed for negation operation"),
        }
    }
    fn not(&self, data_type: &FheSupportedType) -> Self {
        match data_type {
            FheSupportedType::Int64 => {
                let a = self.to_fhe_i64().unwrap();
                FheJsonValue::FheInt64(!a)
            }
            FheSupportedType::Uint64 => {
                let a = self.to_fhe_u64().unwrap();
                FheJsonValue::FheUint64(!a)
            }
            _ => panic!("Unsupported data type passed for bitwise NOT operation"),
        }
    }
}
