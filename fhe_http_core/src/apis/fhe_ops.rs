use crate::configs::fhe_types::FheSupportedType;
use crate::configs::json::FheJsonValue;
use crate::fhe_traits::computable::Computable;
use crate::fhe_traits::value_serialize::FheJsonValueSerialize;

type CompuationResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fhe_add(a: &[u8], b: &[u8], data_type: &str) -> CompuationResult<Vec<u8>> {
    let a: FheJsonValue = FheJsonValueSerialize::deserialize(a);
    let b: FheJsonValue = FheJsonValueSerialize::deserialize(b);
    let data_type: FheSupportedType = FheSupportedType::from_str(data_type);
    let result = a.add(&b, &data_type);
    Ok(result.serialize())
}

pub fn fhe_sub(a: &[u8], b: &[u8], data_type: &str) -> CompuationResult<Vec<u8>> {
    let a: FheJsonValue = FheJsonValueSerialize::deserialize(a);
    let b: FheJsonValue = FheJsonValueSerialize::deserialize(b);
    let data_type: FheSupportedType = FheSupportedType::from_str(data_type);
    let result = a.sub(&b, &data_type);
    Ok(result.serialize())
}

pub fn fhe_mul(a: &[u8], b: &[u8], data_type: &str) -> CompuationResult<Vec<u8>> {
    let a: FheJsonValue = FheJsonValueSerialize::deserialize(a);
    let b: FheJsonValue = FheJsonValueSerialize::deserialize(b);
    let data_type: FheSupportedType = FheSupportedType::from_str(data_type);
    let result = a.mul(&b, &data_type);
    Ok(result.serialize())
}
