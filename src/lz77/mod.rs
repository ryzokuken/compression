#[derive(Debug)]
struct Node {
    offset: usize,
    length: usize,
    symbol: u8,
}

impl Node {
    fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.offset.to_be_bytes());
        out.extend_from_slice(&self.length.to_be_bytes());
        out.push(self.symbol);
        out
    }

    fn deserialize(bytes: &[u8]) -> Node {
        Node {
            offset: usize::from_be_bytes(from_slice(&bytes[0..8])),
            length: usize::from_be_bytes(from_slice(&bytes[8..16])),
            symbol: bytes[16],
        }
    }

    fn blank(symbol: u8) -> Node {
        Node {
            offset: 0,
            length: 0,
            symbol,
        }
    }

    fn new(offset: usize, length: usize) -> Node {
        Node {
            offset,
            length,
            symbol: 0u8,
        }
    }
}

fn from_slice(bytes: &[u8]) -> [u8; 8] {
    let mut arr = [0; 8];
    arr.copy_from_slice(&bytes[..]);
    arr
}

fn deserialize_nodes(data: &[u8]) -> Vec<Node> {
    let mut nodes = Vec::new();
    for block in data.chunks(17) {
        println!("{:?}", block);
        nodes.push(Node::deserialize(block));
    }
    nodes
}

fn serialize_nodes(nodes: &[Node]) -> Vec<u8> {
    let mut out = Vec::new();
    for node in nodes.iter() {
        out.append(&mut node.serialize());
    }
    out
}

pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut nodes = Vec::new();
    nodes.push(Node::blank(data[0]));
    let mut idx = 1;
    'outer: while idx < data.len() {
        let dict = &data[0..idx];
        let mut best = Node::blank(data[idx]);
        let mut len = 1;
        'inner: while len <= idx {
            let buf = &data[idx..idx + len];
            // Naive: break dict into windows of length buf.len() and check if
            // any of them is equal to buf.
            let pos = dict.windows(buf.len()).position(|win| win == buf);
            if pos.is_some() {
                best = Node::new(pos.unwrap(), buf.len());
                // If there's more bytes in the lookahead buffer, keep going.
                if idx + len < data.len() {
                    best.symbol = data[idx + len];
                    len += 1;
                    continue 'inner;
                }
            }
            // If there weren't any matches or we ran out of bytes in the
            // lookahead buffer, call it a day.
            idx += best.length + 1;
            nodes.push(best);
            continue 'outer;
        }
    }
    serialize_nodes(nodes.as_slice())
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        let string = b"abcabdca";
        let _compressed = super::encode(string);
        println!("{:?}", _compressed)
    }
}
