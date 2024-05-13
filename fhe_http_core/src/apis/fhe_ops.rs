use crate::configs::fhe_types::FheSupportedType;
use crate::configs::json::FheJsonValue;
use crate::fhe_traits::computable::Computable;
use crate::fhe_traits::value_serialize::FheJsonValueSerialize;

type CompuationResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn perform_binary_operation<F>(
    a: &Vec<u8>,
    b: &Vec<u8>,
    data_type: &str,
    operation: F,
) -> CompuationResult<Vec<u8>>
where
    F: Fn(&FheJsonValue, &FheJsonValue, &FheSupportedType) -> FheJsonValue,
{
    let a: FheJsonValue = FheJsonValueSerialize::deserialize(a);
    let b: FheJsonValue = FheJsonValueSerialize::deserialize(b);
    let data_type: FheSupportedType = FheSupportedType::from_str(data_type);
    let result = operation(&a, &b, &data_type);
    Ok(result.serialize())
}

fn perform_unary_operation<F>(
    a: &Vec<u8>,
    data_type: &str,
    operation: F,
) -> CompuationResult<Vec<u8>>
where
    F: Fn(&FheJsonValue, &FheSupportedType) -> FheJsonValue,
{
    let a: FheJsonValue = FheJsonValueSerialize::deserialize(a);
    let data_type: FheSupportedType = FheSupportedType::from_str(data_type);
    let result = operation(&a, &data_type);
    Ok(result.serialize())
}

pub fn fhe_add(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.add(b, data_type))
}

pub fn fhe_sub(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.sub(b, data_type))
}

pub fn fhe_mul(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.mul(b, data_type))
}

pub fn fhe_div(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.div(b, data_type))
}

pub fn fhe_rem(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.rem(b, data_type))
}

pub fn fhe_and(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.and(b, data_type))
}

pub fn fhe_or(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.or(b, data_type))
}

pub fn fhe_xor(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.xor(b, data_type))
}

pub fn fhe_shr(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.shr(b, data_type))
}

pub fn fhe_shl(a: &Vec<u8>, b: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.shl(b, data_type))
}

pub fn fhe_neg(a: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_unary_operation(a, data_type, |a, data_type| a.neg(data_type))
}

pub fn fhe_not(a: &Vec<u8>, data_type: &str) -> CompuationResult<Vec<u8>> {
    perform_unary_operation(a, data_type, |a, data_type| a.not(data_type))
}
