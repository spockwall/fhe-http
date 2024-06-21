pub use tfhe::zk::CompactPkePublicParams;
pub use tfhe::{
    CompactPublicKey, FheInt64, FheUint64, ProvenCompactFheInt64, ProvenCompactFheUint64,
};
pub trait ProvenComputable {
    type Output;

    fn add(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn sub(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn mul(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn div(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn rem(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn and(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn or(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn xor(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn neg(
        &self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn not(
        &self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
}

pub trait ProvenShiftable {
    type Output;
    fn shr(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
    fn shl(
        &self,
        o: &Self,
        public_zk_params: &CompactPkePublicParams,
        public_key: &CompactPublicKey,
    ) -> Self::Output;
}

macro_rules! impl_proven_computable {
    ($proven_fhe_ty:ty, $fhe_ty:ty) => {
        impl ProvenComputable for $proven_fhe_ty {
            type Output = $fhe_ty;
            fn add(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() + b.unwrap()
            }
            fn sub(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() - b.unwrap()
            }
            fn mul(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() * b.unwrap()
            }
            fn div(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() / b.unwrap()
            }
            fn rem(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() % b.unwrap()
            }
            fn and(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() & b.unwrap()
            }
            fn or(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() | b.unwrap()
            }
            fn xor(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() ^ b.unwrap()
            }
            fn neg(
                &self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                -a.unwrap()
            }
            fn not(
                &self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                !a.unwrap()
            }
        }
    };
}

macro_rules! impl_proven_shiftable {
    ($proven_fhe_ty:ty, $fhe_ty:ty) => {
        impl ProvenShiftable for $proven_fhe_ty {
            type Output = $fhe_ty;
            fn shr(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() >> b.unwrap()
            }
            fn shl(
                &self,
                o: &Self,
                public_zk_params: &CompactPkePublicParams,
                public_key: &CompactPublicKey,
            ) -> Self::Output {
                let a = self.clone().verify_and_expand(public_zk_params, public_key);
                let b = o.clone().verify_and_expand(public_zk_params, public_key);
                a.unwrap() << b.unwrap()
            }
        }
    };
}

impl_proven_computable!(ProvenCompactFheUint64, FheUint64);
impl_proven_computable!(ProvenCompactFheInt64, FheInt64);
impl_proven_shiftable!(ProvenCompactFheUint64, FheUint64);
