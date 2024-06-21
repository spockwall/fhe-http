pub mod configs {
    pub mod typing;
}

pub mod utils {
    pub mod base64;
    pub mod file_ctl;
    pub mod http;
    pub mod json;
    pub mod rle_compression;
}

pub mod fhe_traits {
    pub mod computable;
    pub mod decryptable;
    pub mod encryptable;
    pub mod serializable;
}

pub mod apis {
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
}

pub extern crate serde;
pub extern crate tfhe;
