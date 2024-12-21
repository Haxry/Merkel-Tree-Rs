use ethers::abi::{encode, Token};
use sha3::{Digest, Keccak256};
use crate::bytes::{BytesLike, HexString, concat_bytes, compare, to_hex};

pub type NodeHash = fn(&BytesLike, &BytesLike) -> HexString;
pub type LeafHash<T> = fn(&T) -> HexString;

pub fn standard_leaf_hash( values: &[Token]) -> HexString {
    
    let abi_encoded = encode(values);
    println!("abi_encoded: {:?}", abi_encoded);

    
    let first_hash = Keccak256::digest(&abi_encoded);
    println!("first_hash: {:?}", first_hash);
    let second_hash = Keccak256::digest(&first_hash);
    println!("second_hash: {:?}", second_hash);
    let result=to_hex(&second_hash.to_vec());
    println!("to_hex: {:?}", result);
    result
}

/// Computes the standard node hash by concatenating and hashing two child nodes.
pub fn standard_node_hash(a: &BytesLike, b: &BytesLike) -> HexString {
    let mut sorted_nodes = vec![a.clone(), b.clone()];
    //sorted_nodes.sort_by(|x, y| compare(x, y));
    let concatenated = concat_bytes(&sorted_nodes);
    println!("concatenated: {:?}", concatenated);
    let hash = Keccak256::digest(&concatenated);
    println!("hash: {:?}", hash);
    let Result= to_hex(&hash.to_vec());
    println!("Result: {:?}", Result);
    Result
}


#[cfg(test)]
mod tests {
    use super::*;
    use ethers::abi::Token;
    use crate::bytes::{to_bytes};

    #[test]
    fn test_standard_leaf_hash() {
        // Define input tokens
        let values = vec![
            Token::Uint(12345.into()),                        // uint256 value
            Token::Address("0xabcdef1234567890abcdef1234567890abcdef12".parse().unwrap()), // address
        ];

        // Compute leaf hash
        let result = standard_leaf_hash(&values);

        // Expected result (calculated externally or from a trusted source)
        let expected = "9f9287cf8093d7756317ac1240f04106c2f7b1bc9b152c2250c964fdcbdf3a85";

        assert_eq!(result, expected, "Standard leaf hash did not match expected value.");
    }

    #[test]
    fn test_standard_node_hash() {
        // Define two child nodes as BytesLike
        let a = to_bytes("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");
        let b = to_bytes("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");

        // Compute node hash
        let result = standard_node_hash(&a, &b);

        // Expected result (calculated externally or from a trusted source)
        let expected = "dac4dff4d3113c4948056d8b50bec1758bff443111e89a0666865d7acb4b572e";

        assert_eq!(result, expected, "Standard node hash did not match expected value.");
    }
}

