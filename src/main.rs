mod huffman;
mod tree;

fn main() {
    let string = "AAAABBBCCD";
    let compressed = huffman::huffman(string);
    println!("{:?}", compressed.into_boxed_slice());
    println!("{:?}", string.as_bytes());
}
