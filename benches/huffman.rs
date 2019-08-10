use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use compression::huffman;

fn huffman_basic(c: &mut Criterion) {
    c.bench_function("Huffman", |b| b.iter(|| huffman::huffman("aaaabbbccd")));
}

criterion_group!(benches, huffman_basic);
criterion_main!(benches);
