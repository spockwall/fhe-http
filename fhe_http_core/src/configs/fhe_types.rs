pub enum FheSupportedType {
    Int64,
    Uint64,
    String,
}

#[allow(dead_code)]
impl FheSupportedType {
    fn as_str(&self) -> &'static str {
        match self {
            FheSupportedType::Int64 => "Int64",
            FheSupportedType::Uint64 => "Uint64",
            FheSupportedType::String => "String",
        }
    }
}
