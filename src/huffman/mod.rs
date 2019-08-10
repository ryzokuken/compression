mod tree;

use bitvec::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use tree::Tree;

const EOF: u8 = 255;

pub fn encode(data: &[u8]) -> (Vec<u8>, Tree) {
    let data = delimit(data);
    let freq = analyze_frequency(data.as_slice());
    let tree = construct_tree(freq);
    let sequences = construct_sequences(&tree);
    let mut bv: BitVec = BitVec::new();
    for x in data.iter() {
        bv.extend(&sequences[&x])
    }
    (bv.into_vec(), tree)
}

pub fn decode(data: &[u8], tree: &Tree) -> Vec<u8> {
    let sequences = construct_sequences(&tree);
    let mut rev_sequences = HashMap::with_capacity(sequences.len());
    for (byte, bv) in sequences.iter() {
        rev_sequences.insert(bv, byte);
    }

    let mut output = Vec::new();
    let data: &BitSlice = BitSlice::from_slice(data);
    let mut idx = 0;
    'outer: while idx < data.len() {
        let mut offset = 0;
        'inner: loop {
            let sub = &data[idx..idx + offset];
            match rev_sequences.get(&BitVec::from(sub)) {
                Some(byte) => match **byte {
                    EOF => break 'outer,
                    _ => {
                        output.push(**byte);
                        idx += offset;
                        break 'inner;
                    }
                },
                None => {
                    offset += 1;
                }
            };
        }
    }
    output
}

fn delimit(data: &[u8]) -> Vec<u8> {
    let mut v = Vec::from(data);
    v.push(EOF);
    v
}

fn construct_sequences(tree: &Tree) -> HashMap<u8, BitVec> {
    traverse_tree(tree, BitVec::new())
}

fn traverse_tree(tree: &Tree, vec: BitVec) -> HashMap<u8, BitVec> {
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
            let left_map = traverse_tree(tree.left.as_ref().unwrap(), left_vec);
            let right_map = traverse_tree(tree.right.as_ref().unwrap(), right_vec);
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
        let string = "aaaabbbccd".as_bytes();
        let (compressed, tree) = super::encode(string);
        assert_eq!(super::decode(compressed.as_slice(), &tree), string);
    }
}
