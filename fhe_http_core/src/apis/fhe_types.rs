use crate::configs::json::NorJsonValue;

// turn i64 into NorJsonValue
impl From<i64> for NorJsonValue {
    fn from(value: i64) -> Self {
        NorJsonValue::Int64(value).into()
    }
}

// turn u64 into NorJsonValue
impl From<u64> for NorJsonValue {
    fn from(value: u64) -> Self {
        NorJsonValue::Uint64(value).into()
    }
}

// turn String into FheJsNorJsonValueonValue
impl From<String> for NorJsonValue {
    fn from(value: String) -> Self {
        NorJsonValue::String(value).into()
    }
}
