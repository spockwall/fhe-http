use tfhe::{prelude::*, ClientKey};
pub trait Decryptable {
    type Output;

    fn val_decrypt(&self, client_key: &ClientKey) -> Self::Output
    where
        Self: Sized;
}
// Define a macro to implement Decryptable for specific types
macro_rules! impl_decryptable {
    ($fhe_ty:ty, $t:ty) => {
        impl Decryptable for $fhe_ty {
            type Output = $t;

            fn val_decrypt(&self, client_key: &ClientKey) -> $t {
                (*self).decrypt(client_key)
            }
        }
    };
}

// Use the macro to implement Encryptable for i64
impl_decryptable!(tfhe::FheInt64, i64);
impl_decryptable!(tfhe::FheUint64, u64);
