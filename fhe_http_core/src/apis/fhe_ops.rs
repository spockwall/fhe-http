use crate::configs::typing::{FheJsonValue, FheSupportedType, SerialFheJsonValue};
use crate::fhe_traits::computable::Computable;
use crate::fhe_traits::value_serialize::FheJsonValueSerialize;

type CompuationResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn perform_binary_operation<F>(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
    operation: F,
) -> CompuationResult<SerialFheJsonValue>
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
    a: &SerialFheJsonValue,
    data_type: &str,
    operation: F,
) -> CompuationResult<SerialFheJsonValue>
where
    F: Fn(&FheJsonValue, &FheSupportedType) -> FheJsonValue,
{
    let a: FheJsonValue = FheJsonValueSerialize::deserialize(a);
    let data_type: FheSupportedType = FheSupportedType::from_str(data_type);
    let result = operation(&a, &data_type);
    Ok(result.serialize())
}

pub fn fhe_add(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.add(b, data_type))
}

pub fn fhe_sub(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.sub(b, data_type))
}

pub fn fhe_mul(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.mul(b, data_type))
}

pub fn fhe_div(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.div(b, data_type))
}

pub fn fhe_rem(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.rem(b, data_type))
}

pub fn fhe_and(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.and(b, data_type))
}

pub fn fhe_or(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.or(b, data_type))
}

pub fn fhe_xor(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.xor(b, data_type))
}

pub fn fhe_shr(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.shr(b, data_type))
}

pub fn fhe_shl(
    a: &SerialFheJsonValue,
    b: &SerialFheJsonValue,
    data_type: &str,
) -> CompuationResult<SerialFheJsonValue> {
    perform_binary_operation(a, b, data_type, |a, b, data_type| a.shl(b, data_type))
}

pub fn fhe_neg(a: &SerialFheJsonValue, data_type: &str) -> CompuationResult<SerialFheJsonValue> {
    perform_unary_operation(a, data_type, |a, data_type| a.neg(data_type))
}

pub fn fhe_not(a: &SerialFheJsonValue, data_type: &str) -> CompuationResult<SerialFheJsonValue> {
    perform_unary_operation(a, data_type, |a, data_type| a.not(data_type))
}
