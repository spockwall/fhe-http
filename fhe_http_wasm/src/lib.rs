pub mod apis {
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
    pub mod key_generator;
    pub mod proven_fhe_ops;
    pub mod serializer;
    pub mod server_key_setter;
}

pub mod configs {
    pub mod error;
    pub mod typing;
    pub mod zk_params;
}

pub mod utils {
    pub mod conversion;
    pub mod web;
}
pub use crate::configs::typing::create_fhe_type;
pub use crate::configs::typing::create_proven_fhe_type;
