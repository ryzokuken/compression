use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use compression::huffman;

fn huffman_basic(c: &mut Criterion) {
    c.bench_function("huffman aaaabbbccd", |b| {
        b.iter(|| huffman::encode(criterion::black_box("aaaabbbccd".as_bytes())))
    });
    c.bench_function("huffman dummy", |b| {
        let kafka = "One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections. The bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream. His room, a proper human room although a little too small, lay peacefully between its four familiar walls. A collection of textile samples lay spread out on the table - Samsa was a travelling salesman - and above it there hung a picture that he had recently cut out of an illustrated magazine and housed in a nice, gilded frame. It showed a lady fitted out with a fur hat and fur boa who sat upright, raising a heavy fur muff that covered the whole of her lower arm towards t";
        b.iter(|| huffman::encode(criterion::black_box(kafka.as_bytes())))
    });
}

criterion_group!(benches, huffman_basic);
criterion_main!(benches);
