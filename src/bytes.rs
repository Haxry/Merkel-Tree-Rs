

use hex::{decode as hex_decode, encode as hex_encode};
use num_bigint::BigInt;


// Type Definitions

/// Represents a hexadecimal string.
pub type HexString = String;

/// Represents bytes-like data.
pub type BytesLike = Vec<u8>;

// Utility Functions

/// Converts a hexadecimal string to bytes.
/// Panics if the input is not valid hexadecimal.
pub fn to_bytes(hex_str: &str) -> BytesLike {
    hex_decode(hex_str).expect("Invalid hexadecimal string")
}

/// Converts bytes to a hexadecimal string.
pub fn to_hex(bytes: &BytesLike) -> HexString {
    hex_encode(bytes)
}

/// Concatenates multiple byte arrays into one.
pub fn concat_bytes(bytes_vec: &[BytesLike]) -> BytesLike {
    bytes_vec.concat()
}


pub fn compare(a: &BytesLike, b: &BytesLike) -> i32 {
    // Convert bytes to hex strings
    let hex_a = to_hex(a);
    let hex_b = to_hex(b);

    // Parse hex strings to BigInt
    let int_a = BigInt::parse_bytes(hex_a.as_bytes(), 16).expect("Invalid hex for BigInt");
    let int_b = BigInt::parse_bytes(hex_b.as_bytes(), 16).expect("Invalid hex for BigInt");

    // Compare the two BigInts
    if int_a > int_b {
        1
    } else if int_a < int_b {
        -1
    } else {
        0
    }
}



// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        let hex_str = "deadbeef";
        let bytes = to_bytes(hex_str);
        assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_to_hex() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let hex_str = to_hex(&bytes);
        assert_eq!(hex_str, "deadbeef");
    }

    #[test]
    fn test_concat_bytes() {
        let a = vec![0xde, 0xad];
        let b = vec![0xbe, 0xef];
        let concatenated = concat_bytes(&[a.clone(), b.clone()]);
        assert_eq!(concatenated, vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_compare() {
        let a = to_bytes("01");
        let b = to_bytes("02");
        assert_eq!(compare(&a, &b), -1);

        let c = to_bytes("ff");
        let d = to_bytes("ff");
        assert_eq!(compare(&c, &d), 0);

        let e = to_bytes("10");
        let f = to_bytes("01");
        assert_eq!(compare(&e, &f), 1);
    }
}
