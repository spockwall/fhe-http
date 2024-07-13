use crate::configs::error::WasmError;
use web_sys::{window, Storage};

pub fn get_storage() -> Result<Storage, WasmError> {
    let window = window().ok_or(WasmError::KeyLoadingError(
        "Error: window is not available".to_string(),
    ))?;

    let storage = window.local_storage().map_or(
        Err(WasmError::KeyLoadingError(
            "Error: local storage is not available".to_string(),
        )),
        |s| {
            Ok(s.ok_or(WasmError::KeyLoadingError(
                "Error: local storage is not available".to_string(),
            ))?)
        },
    )?;

    Ok(storage)
}
