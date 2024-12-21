use core::error;
use crate::bytes::BytesLike;



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







