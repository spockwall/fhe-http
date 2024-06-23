use fhe_http_core::fhe_traits::serializable::ZkSerializable;
use fhe_http_core::tfhe::zk::CompactPkeCrs;
use pyo3::prelude::*;
/// Import the parameters from the shortint module
/// example:
///   use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40
macro_rules! import_shortint_parameters {
    ($(($msg:expr, $carry:expr)), *) => {
        $(
                concat_idents::concat_idents!(
                param_name = PARAM_MESSAGE_,
                $msg,
                _CARRY_,
                $carry,
                _COMPACT_PK_KS_PBS_TUNIFORM_2M40,
                {
                    use fhe_http_core::tfhe::shortint::parameters::param_name;
                }
            );
        )*
    };
}

import_shortint_parameters!(
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (1, 6),
    (2, 0),
    (2, 1),
    (2, 2),
    (2, 3),
    (2, 4),
    (2, 5),
    (2, 6),
    (3, 0),
    (3, 1),
    (3, 2),
    (3, 3),
    (3, 4),
    (3, 5)
);

/// Get PBS param depending on the message and carry
/// range of (msg, carry):
///    (1, 0),(1, 1),(1, 2),(1, 3),(1, 4),(1, 5),(1, 6),
///    (2, 0),(2, 1),(2, 2),(2, 3),(2, 4),(2, 5),(2, 6),
///    (3, 0),(3, 1),(3, 2),(3, 3),(3, 4),(3, 5)
#[pyfunction]
pub fn get_public_zk_params(msg: u8, carry: u8) -> Vec<u8> {
    let max_num_message = 1;
    let params = match (msg, carry) {
        (1, 0) => PARAM_MESSAGE_1_CARRY_0_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 1) => PARAM_MESSAGE_1_CARRY_1_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 2) => PARAM_MESSAGE_1_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 3) => PARAM_MESSAGE_1_CARRY_3_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 4) => PARAM_MESSAGE_1_CARRY_4_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 5) => PARAM_MESSAGE_1_CARRY_5_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (1, 6) => PARAM_MESSAGE_1_CARRY_6_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 0) => PARAM_MESSAGE_2_CARRY_0_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 1) => PARAM_MESSAGE_2_CARRY_1_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 2) => PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 3) => PARAM_MESSAGE_2_CARRY_3_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 4) => PARAM_MESSAGE_2_CARRY_4_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 5) => PARAM_MESSAGE_2_CARRY_5_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (2, 6) => PARAM_MESSAGE_2_CARRY_6_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 0) => PARAM_MESSAGE_3_CARRY_0_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 1) => PARAM_MESSAGE_3_CARRY_1_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 2) => PARAM_MESSAGE_3_CARRY_2_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 3) => PARAM_MESSAGE_3_CARRY_3_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 4) => PARAM_MESSAGE_3_CARRY_4_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        (3, 5) => PARAM_MESSAGE_3_CARRY_5_COMPACT_PK_KS_PBS_TUNIFORM_2M40,
        _ => panic!("Invalid parameters"),
    };
    let crs = CompactPkeCrs::from_shortint_params(params, max_num_message).unwrap();
    let public_zk_params = crs.public_params();
    let res = public_zk_params.try_serialize();
    match res {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    }
}
