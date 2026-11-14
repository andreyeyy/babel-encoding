use rand::{rngs::StdRng, RngExt, SeedableRng};
use std::collections::VecDeque;

fn new_rng() -> impl Iterator<Item = u8> {
    let mut rng = StdRng::seed_from_u64(0);
    std::iter::repeat_with(move || rng.random::<u8>())
}

#[derive(Debug)]
struct BabelEncoding{
    index: u128,
    length: usize,
}
impl BabelEncoding {

    fn from_bytes(data: &[u8]) -> Self {

        let length = data.len();
        let mut index: u128 = 0;

        let mut stream = new_rng();
        let mut window: VecDeque<u8> = stream.by_ref().take(length).collect();

        for byte in stream {
            window.pop_front();
            window.push_back(byte);

            if data.iter().eq(window.iter()) {
                break;
            }
            index += 1;
        }

        BabelEncoding {
            index,
            length,
        }
    }

    fn decode(self) -> Vec<u8> {
        let mut stream = new_rng();
        let mut i = self.index + 1;

        while i > 0 {
            stream.next();
            i -= 1;
        }
        stream.take(self.length).collect()
    }
}

fn main() {
    let bytes: Vec<u8> = vec![12, 13, 14, 15, 16];
    println!("Input: {:?}", bytes);

    let encoding = BabelEncoding::from_bytes(&bytes);
    println!("{:?}", encoding);

    let decoded = encoding.decode();
    println!("Decoded: {:?}", decoded);
}

