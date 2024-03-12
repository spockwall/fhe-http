extern crate base64;
use base64::{engine::general_purpose::STANDARD, DecodeError, Engine as _};

mod base64_tests {
    #[cfg(test)]
    mod tests {
        use super::super::*;
        #[test]
        fn test_encode() {
            let input: &str = "Hello, World!";
            let expected: &str = "SGVsbG8sIFdvcmxkIQ==";
            let result: String = STANDARD.encode(input);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_decode() {
            let input: &str = "SGVsbG8sIFdvcmxkIQ==";
            let expected: &str = "Hello, World!";
            let result: Result<Vec<u8>, DecodeError> = STANDARD.decode(input);
            assert_eq!(result.unwrap(), expected.as_bytes());
        }
    }
}
