mod tree;

use bitvec::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use tree::Tree;

pub fn encode(data: &[u8]) -> Vec<u8> {
    let sequence = preprocess(data);
    let mut bit_vec: BitVec = BitVec::new();
    for x in data.iter() {
        bit_vec.extend(&sequence[&x])
    }
    bit_vec.into_vec()
}

// pub fn decode(text: &[u8]) -> Vec<u8> {
//     let sequence = preprocess(data);
// }

fn preprocess(data: &[u8]) -> HashMap<u8, BitVec> {
    construct_sequences(construct_tree(analyze_frequency(data)))
}

fn construct_sequences(tree: Tree) -> HashMap<u8, BitVec> {
    traverse_tree(tree, BitVec::new())
}

fn traverse_tree(tree: Tree, vec: BitVec) -> HashMap<u8, BitVec> {
    match tree.data {
        Some(byte) => {
            let mut map = HashMap::with_capacity(1);
            map.insert(byte, vec);
            map
        }
        None => {
            let mut left_vec = vec.clone();
            left_vec.push(false);
            let mut right_vec = vec.clone();
            right_vec.push(true);
            let left_map = traverse_tree(*tree.left.unwrap(), left_vec);
            let right_map = traverse_tree(*tree.right.unwrap(), right_vec);
            let mut map = HashMap::with_capacity(left_map.len() + right_map.len());
            map.extend(left_map);
            map.extend(right_map);
            map
        }
    }
}

fn construct_tree(freq: HashMap<u8, u32>) -> Tree {
    if freq.len() <= 0 {
        return Tree::blank();
    }
    let mut heap = BinaryHeap::new();
    for (byte, f) in freq.iter() {
        heap.push(Reverse((*f, Tree::new(*byte))));
    }
    while heap.len() > 1 {
        let first = heap.pop().unwrap().0;
        let second = heap.pop().unwrap().0;
        heap.push(Reverse((
            first.0 + second.0,
            Tree::blank().set_left(first.1).set_right(second.1),
        )));
    }
    (heap.pop().unwrap().0).1
}

fn analyze_frequency(text: &[u8]) -> HashMap<u8, u32> {
    let mut freq = HashMap::new();
    for byte in text.iter() {
        *freq.entry(*byte).or_insert(0) += 1;
    }
    freq
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let string = "aaaabbbccd";
        let out = &[0x0Au8, 0xBFu8, 0xC0u8];
        assert_eq!(super::encode(string.as_bytes()).as_slice(), out);
    }
}
