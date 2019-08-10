mod tree;

use tree::Tree;
use bitvec::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn huffman(text: &str) -> Vec<u8> {
    let freq = analyze_frequency(text);
    let tree = construct_tree(freq);
    let sequence = construct_sequence(tree);
    let mut bit_vec: BitVec = BitVec::new();
    for x in text.chars() {
        bit_vec.extend(&sequence[&x])
    }
    bit_vec.into_vec()
}

fn construct_sequence(tree: Tree) -> HashMap<char, BitVec> {
    traverse_tree(tree, BitVec::new())
}

fn traverse_tree(tree: Tree, vec: BitVec) -> HashMap<char, BitVec> {
    match tree.data {
        Some(ch) => {
            let mut map = HashMap::with_capacity(1);
            map.insert(ch, vec);
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

fn construct_tree(freq: HashMap<char, u32>) -> Tree {
    if freq.len() <= 0 {
        return Tree::blank();
    }
    let mut heap = BinaryHeap::new();
    for (ch, f) in freq.iter() {
        heap.push(Reverse((*f, Tree::new(*ch))));
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

fn analyze_frequency(text: &str) -> HashMap<char, u32> {
    let mut freq = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}
