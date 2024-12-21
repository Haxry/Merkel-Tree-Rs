
//use hexstring::HexString;

use crate::bytes::{BytesLike,HexString,to_hex,to_bytes};
use crate::hash::{standard_leaf_hash, standard_node_hash,NodeHash,LeafHash};



pub fn left_child_index(i: usize) -> usize {
    2 * i + 1
}

pub fn right_child_index(i: usize) -> usize {
    2 * i + 2
}

pub fn parent_index(i: usize) -> Option<usize> {
    if i > 0 {
        Some((i - 1) / 2)
    } else {
        None 
    }
}

pub fn sibling_index(i: usize) -> Option<usize> {
    if i == 0 {
        None 
    } else if i % 2 == 0 {
        Some(i - 1)
    } else {
        Some(i + 1)
    }
}

pub fn is_tree_node(i: usize, tree_size: usize) -> bool {
    i < tree_size
}

pub fn is_internal_node(i: usize, tree_size: usize) -> bool {
    is_tree_node(left_child_index(i), tree_size)
}

pub fn is_leaf_node(i: usize, tree_size: usize) -> bool {
    is_tree_node(i, tree_size) && !is_internal_node(i, tree_size)
}





pub fn make_merkle_tree(leaves: &[BytesLike], node_hash: NodeHash) -> Vec<HexString> {
    let num_leaves = leaves.len();
    let tree_size = 2 * num_leaves - 1;
    let mut tree = vec![String::new(); tree_size];

    
    for (i, leaf) in leaves.iter().enumerate() {
        tree[tree_size - 1 - i] = to_hex(leaf);
    }

    // Compute internal nodes
    for i in (0..(tree_size - num_leaves)).rev() {
        let left = &tree[left_child_index(i)];
        let right = &tree[right_child_index(i)];
        tree[i] = node_hash(&hex::decode(left).expect("Invalid hex"), &hex::decode(right).expect("Invalid hex"));
        // println!("i is {:?}", i);
        // println!("tree[i]: {:?}", tree[i]);

    }

    tree
}


pub fn get_proof(tree: &[HexString], mut index: usize) -> Vec<HexString> {
    
    let tree_bytes: Vec<BytesLike> = tree.iter().map(|hex_str| to_bytes(hex_str)).collect();

    let mut proof = Vec::new();

    while index > 0 {
        let sibling = sibling_index(index); 
        if let Some(sibling_index) = sibling {
            proof.push(to_hex(&tree_bytes[sibling_index]));
        }
        index = parent_index(index).unwrap(); 
    }

    proof
}




#[cfg(test)]
mod tests {
    use super::*;
    

    

    #[test]
    fn test_make_merkle_tree() {
        
        let leaves = vec![
            to_bytes("0xabcdef"),
            to_bytes("0x123456"),
            to_bytes("0x789abc"),
            to_bytes("0xdeadbeef"),
        ];

        
        let expected_tree = vec![
            "75baacf0502fe4b13d10e8c703a560b143a4caf924c729309db81761551c9e9d".to_string(),          // Root node
            "5a933e4f700f4e57610654229a5fd76c1b46f3ed37b5af1bd12ef7258edd4b79".to_string(),   // Left internal node
            "3614eb8d39b2e39b06d70e3198afd9170a4a35726d55ca9757af151353da27ca".to_string(),   // Right internal node
            "deadbeef".to_string(),
            "789abc".to_string(),
            "123456".to_string(),
            "abcdef".to_string(),                     
         ];

         let proof = get_proof(&expected_tree, 3);
         println!("proof: {:?}", proof);
//proof: ["789abc", "3614eb8d39b2e39b06d70e3198afd9170a4a35726d55ca9757af151353da27ca"]
        
        let tree = make_merkle_tree(&leaves, standard_node_hash);
        //println!("tree: {:?}", tree);

        
        assert_eq!(tree.len(), 2 * leaves.len() - 1, "Tree size mismatch.");

        
        for (i, expected) in expected_tree.iter().enumerate() {
            assert_eq!(
                tree[i],
                *expected,
                "Node {} mismatch. Expected: {}, Got: {}",
                i,
                expected,
                tree[i]
            );
        }
    }
}







