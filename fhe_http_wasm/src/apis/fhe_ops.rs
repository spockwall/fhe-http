//! This module provides python api for FHE operations
//! Input and output are serialized as Vec<u8> and WasmFheType

use crate::configs::error::OperationError;
use crate::configs::typing::WasmFheType;
use fhe_http_core::apis::fhe_ops::*;
use wasm_bindgen::prelude::*;

/// Create python api for binary operations as WasmFheType being data type
///
/// Input arguments:
///     a: Vec<u8> - first operand
///     b: Vec<u8> - second operand
///     data_type: WasmFheType (see configs/typing.rs for more details)
///
/// Returns:
///     Vec<u8> - serialized result of the operation
#[macro_export]
macro_rules! impl_binary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(
            a: Vec<u8>,
            b: Vec<u8>,
            data_type: WasmFheType,
        ) -> Result<Vec<u8>, JsValue> {
            if let Ok(res) = ($method)(&a, &b, &data_type) {
                Ok(res)
            } else {
                Err(JsValue::from_str(&format!(
                    "{}",
                    OperationError::BinaryOpError("Operation failed".to_string())
                )))
            }
        }
    };
}

/// Create python api for unary operations as WasmFheType being data type
///
/// Input arguments:
///     a: Vec<u8> - operand
///     data_type: WasmFheType (see configs/typing.rs for more details)
///     
/// Returns:
///     Vec<u8> - serialized result of the operation
///     
macro_rules! impl_unary_py_fhe_ops {
    ($func_name:ident, $method:ident) => {
        pub fn $func_name(a: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
            if let Ok(res) = ($method)(&a, &data_type) {
                Ok(res)
            } else {
                Err(JsValue::from_str(&format!(
                    "{}",
                    OperationError::UnaryOpError("Unary Operation failed".to_string())
                )))
            }
        }
    };
}

#[wasm_bindgen]
pub struct FheOps {}

#[wasm_bindgen]
impl FheOps {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        FheOps {}
    }

    pub fn add(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(add, fhe_add);
        add(a, b, data_type)
    }

    pub fn sub(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(sub, fhe_sub);
        sub(a, b, data_type)
    }

    pub fn mul(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(mul, fhe_mul);
        mul(a, b, data_type)
    }

    pub fn div(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(div, fhe_div);
        div(a, b, data_type)
    }

    pub fn and(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(and, fhe_and);
        and(a, b, data_type)
    }

    pub fn or(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(or, fhe_or);
        or(a, b, data_type)
    }

    pub fn xor(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(xor, fhe_xor);
        xor(a, b, data_type)
    }

    pub fn shr(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(shr, fhe_shr);
        shr(a, b, data_type)
    }

    pub fn shl(&self, a: Vec<u8>, b: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_binary_py_fhe_ops!(shl, fhe_shl);
        shl(a, b, data_type)
    }

    pub fn not(&self, a: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_unary_py_fhe_ops!(not, fhe_not);
        not(a, data_type)
    }

    pub fn neg(&self, a: Vec<u8>, data_type: WasmFheType) -> Result<Vec<u8>, JsValue> {
        impl_unary_py_fhe_ops!(neg, fhe_neg);
        neg(a, data_type)
    }
}
