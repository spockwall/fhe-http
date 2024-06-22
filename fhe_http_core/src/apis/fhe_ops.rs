use crate::configs::typing::{CompuationResult, FheValue};
use crate::fhe_traits::computable::Computable;
use crate::fhe_traits::computable::Shiftable;
use crate::fhe_traits::serializable::FheValueSerializable;
use tfhe::{FheInt64, FheUint64};

macro_rules! generate_binary_operation {
    ($fn_name:ident, $op_method:ident) => {
        pub fn $fn_name(
            a: &Vec<u8>,
            b: &Vec<u8>,
            data_type: &FheValue,
        ) -> CompuationResult<Vec<u8>> {
            match data_type {
                FheValue::Int64 => {
                    let a = FheInt64::try_deserialize(a).unwrap();
                    let b = FheInt64::try_deserialize(b).unwrap();
                    let result = a.$op_method(&b);
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
                FheValue::Uint64 => {
                    let a = FheUint64::try_deserialize(a).unwrap();
                    let b = FheUint64::try_deserialize(b).unwrap();
                    let result = a.$op_method(&b);
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
            }
        }
    };
}

macro_rules! generate_unary_operation {
    ($fn_name:ident, $op_method:ident) => {
        pub fn $fn_name(a: &Vec<u8>, data_type: &FheValue) -> CompuationResult<Vec<u8>> {
            match data_type {
                FheValue::Int64 => {
                    let a = FheInt64::try_deserialize(a).unwrap();
                    let result = a.$op_method();
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
                FheValue::Uint64 => {
                    let a = FheUint64::try_deserialize(a).unwrap();
                    let result = a.$op_method();
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
            }
        }
    };
}

macro_rules! generate_binary_shift_operation {
    ($fn_name:ident, $op_method:ident) => {
        pub fn $fn_name(
            a: &Vec<u8>,
            b: &Vec<u8>,
            data_type: &FheValue,
        ) -> CompuationResult<Vec<u8>> {
            match data_type {
                FheValue::Uint64 => {
                    let a: FheUint64 = FheValueSerializable::try_deserialize(a).unwrap();
                    let b: FheUint64 = FheValueSerializable::try_deserialize(b).unwrap();
                    let result = a.$op_method(&b);
                    Ok(result.try_serialize().expect("Failed to serialize"))
                }
                _ => {
                    panic!("Operation is not supported for Int64");
                }
            }
        }
    };
}

generate_binary_operation!(fhe_add, add);
generate_binary_operation!(fhe_sub, sub);
generate_binary_operation!(fhe_mul, mul);
generate_binary_operation!(fhe_div, div);
generate_binary_operation!(fhe_rem, rem);
generate_binary_operation!(fhe_and, and);
generate_binary_operation!(fhe_or, or);
generate_binary_operation!(fhe_xor, xor);
generate_binary_shift_operation!(fhe_shr, shr);
generate_binary_shift_operation!(fhe_shl, shl);
generate_unary_operation!(fhe_neg, neg);
generate_unary_operation!(fhe_not, not);
