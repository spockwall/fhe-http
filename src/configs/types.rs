pub enum SupportedTypes {
    Uint32(u32),
    Int32(i32),
    Uint128(u128),
    Int128(i128),
    String(String),
}

pub fn is_supported_type(_type: &str) -> bool {
    match _type {
        "Uint32" => true,
        "Int32" => true,
        "Uint128" => true,
        "Int128" => true,
        "String" => true,
        _ => false,
    }
}
