use tfhe::prelude::FheTryEncrypt;
use tfhe::ClientKey;

type FheError = Box<dyn std::error::Error>;
pub trait Encryptable {
    type Output;

    fn val_encrypt(&self, client_key: &ClientKey) -> Result<Self::Output, FheError>
    where
        Self: Sized;

    // type check
    fn type_name(&self) -> &'static str;

    // convert to json value
    fn to_json_value(&self) -> serde_json::Value;
}

// Now, define the macro to implement Encryptable for specific types
macro_rules! impl_encryptable {
    ($t:ty, $fhe_ty:ty) => {
        impl Encryptable for $t {
            type Output = $fhe_ty;

            fn val_encrypt(&self, client_key: &ClientKey) -> Result<Self::Output, FheError> {
                match FheTryEncrypt::try_encrypt(*self, client_key) {
                    Ok(encrypted) => Ok(encrypted),
                    Err(e) => Err(e.into()),
                }
            }
            fn type_name(&self) -> &'static str {
                stringify!($t)
            }

            fn to_json_value(&self) -> serde_json::Value {
                serde_json::Value::Number(serde_json::Number::from(*self))
            }
        }
    };
}

// Use the macro to implement Encryptable for i64 and u64
impl_encryptable!(i64, tfhe::FheInt64);
impl_encryptable!(u64, tfhe::FheUint64);
