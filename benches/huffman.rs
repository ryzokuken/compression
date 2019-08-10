use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use compression::huffman;

fn huffman_basic(c: &mut Criterion) {
    c.bench_function("Huffman", |b| {
        b.iter(|| huffman::huffman(criterion::black_box("aaaabbbccd")))
    });
}

criterion_group!(benches, huffman_basic);
criterion_main!(benches);
