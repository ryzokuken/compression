use crate::tree::Tree;
use bitvec::prelude::*;
use std::collections::HashMap;

pub fn huffman(text: &str) -> BitVec {
    let freq = analyze_frequency(text);
    let tree = construct_tree(freq);
    let sequence = construct_sequence(tree);
    let mut bit_vec: BitVec = BitVec::new();
    for x in text.chars() {
        bit_vec.extend(&sequence[&x])
    }
    bit_vec
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
    let mut tree_vec = Vec::new();
    for (ch, f) in freq.iter() {
        tree_vec.push((Tree::new(*ch), *f))
    }
    while tree_vec.len() > 1 {
        tree_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let first = tree_vec.pop().unwrap();
        let second = tree_vec.pop().unwrap();
        let merged = Tree::blank().set_left(first.0).set_right(second.0);
        tree_vec.push((merged, first.1 + second.1));
    }
    tree_vec.pop().unwrap().0
}

fn analyze_frequency(text: &str) -> HashMap<char, u32> {
    let mut freq = HashMap::new();
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}
