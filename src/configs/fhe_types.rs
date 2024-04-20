use serde_json::Value;

// Trait for validating and converting values
pub trait FheSupportedType<T> {
    fn is_val_supported_type(val: &Value) -> bool;
    fn val_to_supported_type(value: &Value) -> Result<T, String>;
}

impl FheSupportedType<i32> for i32 {
    fn is_val_supported_type(val: &Value) -> bool {
        match val {
            Value::Number(n) => n
                .as_i64()
                .map(|num| (i32::MIN as i64) <= num && num <= (i32::MAX as i64))
                .unwrap_or(false),
            _ => false,
        }
    }

    fn val_to_supported_type(value: &Value) -> Result<i32, String> {
        match value.as_i64() {
            Some(num) if (i32::MIN as i64) <= num && num <= (i32::MAX as i64) => Ok(num as i32),
            _ => Err("Unsupported or invalid value for i32".to_string()),
        }
    }
}

impl FheSupportedType<u32> for u32 {
    fn is_val_supported_type(val: &Value) -> bool {
        match val {
            Value::Number(n) => n
                .as_u64()
                .map(|num| num <= (u32::MAX as u64))
                .unwrap_or(false),
            _ => false,
        }
    }

    fn val_to_supported_type(value: &Value) -> Result<u32, String> {
        match value.as_u64() {
            Some(num) if num <= (u32::MAX as u64) => Ok(num as u32),
            _ => Err("Unsupported or invalid value for u32".to_string()),
        }
    }
}

impl FheSupportedType<i64> for i64 {
    fn is_val_supported_type(val: &Value) -> bool {
        match val {
            Value::Number(n) => n.as_i64().map(|_| true).unwrap_or(false),
            _ => false,
        }
    }

    fn val_to_supported_type(value: &Value) -> Result<i64, String> {
        match value.as_i64() {
            Some(num) => Ok(num),
            _ => Err("Unsupported or invalid value for i64".to_string()),
        }
    }
}

impl FheSupportedType<u64> for u64 {
    fn is_val_supported_type(val: &Value) -> bool {
        match val {
            Value::Number(n) => n.as_u64().map(|_| true).unwrap_or(false),
            _ => false,
        }
    }

    fn val_to_supported_type(value: &Value) -> Result<u64, String> {
        match value.as_u64() {
            Some(num) => Ok(num),
            _ => Err("Unsupported or invalid value for u64".to_string()),
        }
    }
}

// Example implementations for u32, i64, etc., would be similar

impl FheSupportedType<String> for String {
    fn is_val_supported_type(val: &Value) -> bool {
        val.is_string()
    }

    fn val_to_supported_type(value: &Value) -> Result<String, String> {
        match value.as_str() {
            Some(s) => Ok(s.to_string()),
            None => Err("Value is not a string".to_string()),
        }
    }
}

// Global function to check if any type is supported
pub fn is_val_supported_type(val: &Value) -> bool {
    i32::is_val_supported_type(val)
        || u32::is_val_supported_type(val)
        || i64::is_val_supported_type(val)
        || u64::is_val_supported_type(val)
        || String::is_val_supported_type(val)
}
