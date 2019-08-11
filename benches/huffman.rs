use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use compression::huffman;

fn huffman_encode(c: &mut Criterion) {
    c.bench_function("huffman encode aaaabbbccd", |b| {
        b.iter(|| huffman::encode(criterion::black_box("aaaabbbccd".as_bytes())))
    });
    c.bench_function("huffman encode dummy", |b| {
        let kafka = "One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections. The bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream. His room, a proper human room although a little too small, lay peacefully between its four familiar walls. A collection of textile samples lay spread out on the table - Samsa was a travelling salesman - and above it there hung a picture that he had recently cut out of an illustrated magazine and housed in a nice, gilded frame. It showed a lady fitted out with a fur hat and fur boa who sat upright, raising a heavy fur muff that covered the whole of her lower arm towards t";
        b.iter(|| huffman::encode(criterion::black_box(kafka.as_bytes())))
    });
}

fn huffman_decode(c: &mut Criterion) {
    let (basic, btree) = huffman::encode("aaaabbbccd".as_bytes());
    c.bench_function("huffman decode aaaabbbccd", move |b| {
        b.iter(|| {
            huffman::decode(
                criterion::black_box(basic.as_slice()),
                criterion::black_box(&btree),
            )
        })
    });
    let (kafka, ktree) = huffman::encode("One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections. The bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream. His room, a proper human room although a little too small, lay peacefully between its four familiar walls. A collection of textile samples lay spread out on the table - Samsa was a travelling salesman - and above it there hung a picture that he had recently cut out of an illustrated magazine and housed in a nice, gilded frame. It showed a lady fitted out with a fur hat and fur boa who sat upright, raising a heavy fur muff that covered the whole of her lower arm towards t".as_bytes());
    c.bench_function("huffman decode dummy", move |b| {
        b.iter(|| {
            huffman::decode(
                criterion::black_box(kafka.as_slice()),
                criterion::black_box(&ktree),
            )
        })
    });
}

fn huffman_encode_decode(c: &mut Criterion) {
    c.bench_function("huffman encode decode aaaabbbccd", move |b| {
        b.iter(|| {
            let (basic, btree) = huffman::encode(criterion::black_box("aaaabbbccd".as_bytes()));
            huffman::decode(
                criterion::black_box(basic.as_slice()),
                criterion::black_box(&btree),
            )
        })
    });
    c.bench_function("huffman encode decode dummy", move |b| {
        b.iter(|| {
            let (kafka, ktree) = huffman::encode(criterion::black_box("One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections. The bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream. His room, a proper human room although a little too small, lay peacefully between its four familiar walls. A collection of textile samples lay spread out on the table - Samsa was a travelling salesman - and above it there hung a picture that he had recently cut out of an illustrated magazine and housed in a nice, gilded frame. It showed a lady fitted out with a fur hat and fur boa who sat upright, raising a heavy fur muff that covered the whole of her lower arm towards t".as_bytes()));
            huffman::decode(
                criterion::black_box(kafka.as_slice()),
                criterion::black_box(&ktree),
            )
        })
    });
}

criterion_group!(
    benches,
    huffman_encode,
    huffman_decode,
    huffman_encode_decode
);
criterion_main!(benches);
