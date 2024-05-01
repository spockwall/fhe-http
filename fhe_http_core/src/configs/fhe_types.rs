pub enum FheSupportedType {
    Int64,
    Uint64,
    String,
}

#[allow(dead_code)]
impl FheSupportedType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FheSupportedType::Int64 => "Int64",
            FheSupportedType::Uint64 => "Uint64",
            FheSupportedType::String => "String",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "Int64" => FheSupportedType::Int64,
            "Uint64" => FheSupportedType::Uint64,
            "String" => FheSupportedType::String,
            _ => panic!("Unsupported type"),
        }
    }
}
