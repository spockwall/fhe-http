pub mod apis {
    pub mod base64;
    pub mod fhe;
    pub mod key_generator;
    pub mod serializer;
}
pub mod configs {
    pub mod error;
    pub mod typing;
    pub mod zk_params;
}

pub mod utils {
    pub mod web;
}
pub use crate::configs::typing::create_fhe_type;
pub use crate::configs::typing::create_proven_fhe_type;
