use crate::configs::typing::CompuationResult;
use crate::fhe_traits::computable::{Computable, Shiftable};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::serializable::FheValueSerializable;

fn perform_binary_operation<T, F>(
    a: &Vec<u8>,
    b: &Vec<u8>,
    operation: F,
) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable,
    F: Fn(&T, &T) -> T,
{
    let a: T = FheValueSerializable::try_deserialize(a).unwrap();
    let b: T = FheValueSerializable::try_deserialize(b).unwrap();
    let result = operation(&a, &b);
    Ok(result.try_serialize().expect("Failed to serialize"))
}

fn perform_unary_operation<T, F>(a: &Vec<u8>, operation: F) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable,
    F: Fn(&T) -> T,
{
    let a: T = FheValueSerializable::try_deserialize(a).unwrap();
    let result = operation(&a);
    Ok(result.try_serialize().expect("Failed to serialize"))
}

pub fn fhe_add<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.add(b))
}

pub fn fhe_sub<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.sub(b))
}

pub fn fhe_mul<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.mul(b))
}

pub fn fhe_div<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.div(b))
}
pub fn fhe_rem<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.rem(b))
}

pub fn fhe_and<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.and(b))
}

pub fn fhe_or<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.or(b))
}

pub fn fhe_xor<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.xor(b))
}
pub fn fhe_shr<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Shiftable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.shr(b))
}

pub fn fhe_shl<T>(a: &Vec<u8>, b: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Shiftable,
{
    perform_binary_operation(a, b, |a: &T, b: &T| a.shl(b))
}
pub fn fhe_neg<T>(a: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_unary_operation(a, |a: &T| a.neg())
}

pub fn fhe_not<T>(a: &Vec<u8>) -> CompuationResult<Vec<u8>>
where
    T: Decryptable + FheValueSerializable + Computable,
{
    perform_unary_operation(a, |a: &T| a.not())
}
