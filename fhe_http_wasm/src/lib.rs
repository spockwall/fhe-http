pub mod apis {
    pub mod fhe;
}
pub mod configs {
    pub mod typing;
}

pub use crate::configs::typing::create_fhe_type;
pub use crate::configs::typing::create_proven_fhe_type;
