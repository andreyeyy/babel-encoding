use super::*;

#[test]
fn empty_slice() {
    let input: Vec<u8> = vec![];

    let encoding = BabelEncoding::from_bytes(&input);

    assert_eq!(
        encoding,
        BabelEncoding {
            index: 0,
            length: 0,
        }
    );
}

#[test]
fn empty_slice_decoding() {
    let input: Vec<u8> = vec![];

    let encoding = BabelEncoding::from_bytes(&input);
    let original = encoding.decode();

    assert_eq!(input, original);
}

#[test]
fn stream_prefix_encodes_at_zero() {
    let input: Vec<u8> = new_rng().take(4).collect();

    let encoding = BabelEncoding::from_bytes(&input);

    assert_eq!(
        encoding,
        BabelEncoding {
            index: 0,
            length: 4,
        }
    );
    assert_eq!(encoding.decode(), input);
}

#[test]
fn stream_slice_encodes_at_known_offset() {
    let offset = 100;
    let length = 8;
    let input: Vec<u8> = new_rng().skip(offset).take(length).collect();

    let encoding = BabelEncoding::from_bytes(&input);

    assert_eq!(
        encoding,
        BabelEncoding {
            index: offset as u128,
            length,
        }
    );
    assert_eq!(encoding.decode(), input);
}

#[test]
fn repeated_bytes_encode_and_decode() {
    let offset = 20;
    let length = 3;
    let input: Vec<u8> = new_rng().skip(offset).take(length).collect();

    let encoding = BabelEncoding::from_bytes(&input);

    assert_eq!(encoding.length, length);
    assert_eq!(encoding.decode(), input);
}
