use tfhe::prelude::FheTryEncrypt;
use tfhe::zk::{CompactPkeCrs, ZkComputeLoad};
use tfhe::{
    ClientKey, CompactPublicKey, FheInt64, FheUint64, ProvenCompactFheInt64, ProvenCompactFheUint64,
};
type FheError = Box<dyn std::error::Error>;
pub trait Encryptable {
    type Output;

    fn val_encrypt(&self, client_key: &ClientKey) -> Result<Self::Output, FheError>
    where
        Self: Sized;

    // convert to json value
    fn to_json_value(&self) -> serde_json::Value;
}

pub trait ProvenEncryptable {
    type ProvenOutput;
    fn proven_encrypt(
        &self,
        public_zk_params: &CompactPkeCrs,
        public_key: &CompactPublicKey,
    ) -> Result<Self::ProvenOutput, FheError>
    where
        Self: Sized;

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

            fn to_json_value(&self) -> serde_json::Value {
                serde_json::Value::Number(serde_json::Number::from(*self))
            }
        }
    };
}

macro_rules! impl_proven_encryptable {
    ($t:ty, $proven_fhe_ty:ty) => {
        impl ProvenEncryptable for $t {
            type ProvenOutput = $proven_fhe_ty;
            fn proven_encrypt(
                &self,
                public_zk_params: &CompactPkeCrs,
                public_key: &CompactPublicKey,
            ) -> Result<Self::ProvenOutput, FheError> {
                let a = <$proven_fhe_ty>::try_encrypt(
                    *self,
                    public_zk_params.public_params(),
                    &public_key,
                    ZkComputeLoad::Proof,
                );
                match a {
                    Ok(encrypted) => Ok(encrypted),
                    Err(e) => Err(e.into()),
                }
            }

            fn to_json_value(&self) -> serde_json::Value {
                serde_json::Value::Number(serde_json::Number::from(*self))
            }
        }
    };
}

// Use the macro to implement Encryptable for i64 and u64
impl_encryptable!(i64, FheInt64);
impl_encryptable!(u64, FheUint64);
impl_proven_encryptable!(i64, ProvenCompactFheInt64);
impl_proven_encryptable!(u64, ProvenCompactFheUint64);
