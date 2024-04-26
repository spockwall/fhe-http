use crate::configs::fhe_types::FheSupportedType;
use crate::configs::json::FheJsonValue;
use crate::fhe_traits::computable::Computable;
pub fn fhe_add(a: &FheJsonValue, b: &FheJsonValue, data_type: &FheSupportedType) -> FheJsonValue {
    a.add(b, data_type)
}

pub fn fhe_sub(a: &FheJsonValue, b: &FheJsonValue, data_type: &FheSupportedType) -> FheJsonValue {
    a.sub(b, data_type)
}

pub fn fhe_mul(a: &FheJsonValue, b: &FheJsonValue, data_type: &FheSupportedType) -> FheJsonValue {
    a.mul(b, data_type)
}
