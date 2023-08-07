#![feature(test)]

pub fn encode(bytes: Vec<u8>) -> String {
    let remainder = bytes.len() % 3;

    let mut res = String::with_capacity(4 * (bytes.len() / 3) + if remainder == 0 { 0 } else { 4 });

    for i in 0..(bytes.len() / 3) {
        res += ENCODE_TABLE[((bytes[3 * i] & 0b11111100) >> 2) as usize];
        res += ENCODE_TABLE
            [(((bytes[3 * i] & 0b11) << 4) | ((bytes[3 * i + 1] & 0b11110000) >> 4)) as usize];
        res += ENCODE_TABLE[(((bytes[3 * i + 1] & 0b1111) << 2)
            | ((bytes[3 * i + 2] & 0b11000000) >> 6)) as usize];
        res += ENCODE_TABLE[(bytes[3 * i + 2] & 0b111111) as usize];
    }

    match remainder {
        0 => return res,
        1 => {
            res += ENCODE_TABLE[((bytes[bytes.len() - 1] & 0b11111100) >> 2) as usize];
            res += ENCODE_TABLE[((bytes[bytes.len() - 1] & 0b11) << 4) as usize];
            res += "==";
            return res;
        }
        2 => {
            res += ENCODE_TABLE[((bytes[bytes.len() - 2] & 0b11111100) >> 2) as usize];
            res += ENCODE_TABLE[(((bytes[bytes.len() - 2] & 0b11) << 4)
                | ((bytes[bytes.len() - 1] & 0b11110000) >> 4))
                as usize];
            res += ENCODE_TABLE[((bytes[bytes.len() - 1] & 0b1111) << 2) as usize];
            res += "=";
            return res;
        }
        _ => unreachable!("Invalid input"),
    }
}

fn decode(input: String) -> Vec<u8> {
    if input.len() == 0 {
        return vec![];
    }

    let mut input_bytes = input.into_bytes();
    if input_bytes.len() % 4 != 0 {
        panic!("Invalid base64 string");
    }

    let mut result = Vec::with_capacity((3 * input_bytes.len()) / 4);

    let mut first_extra_value: Option<u8> = None;
    let mut second_extra_value: Option<u8> = None;

    if input_bytes[input_bytes.len() - 2] == 61 {
        let first_sextet = DECODE_TABLE[input_bytes[input_bytes.len() - 4] as usize];
        let second_sextet = DECODE_TABLE[input_bytes[input_bytes.len() - 3] as usize];

        first_extra_value = Some((first_sextet << 2) | ((second_sextet & 0b110000) >> 4));

        input_bytes.truncate(input_bytes.len() - 4);
    } else if input_bytes[input_bytes.len() - 1] == 61 {
        let first_sextet = DECODE_TABLE[input_bytes[input_bytes.len() - 4] as usize];
        let second_sextet = DECODE_TABLE[input_bytes[input_bytes.len() - 3] as usize];
        let third_sextet = DECODE_TABLE[input_bytes[input_bytes.len() - 2] as usize];

        first_extra_value = Some((first_sextet << 2) | ((second_sextet & 0b110000) >> 4));
        second_extra_value =
            Some(((second_sextet & 0b1111) << 4) | ((third_sextet & 0b111100) >> 2));

        input_bytes.truncate(input_bytes.len() - 4);
    }

    for i in 0..input_bytes.len() / 4 {
        let first_sextet = DECODE_TABLE[input_bytes[i * 4] as usize];
        let second_sextet = DECODE_TABLE[input_bytes[i * 4 + 1] as usize];
        let third_sextet = DECODE_TABLE[input_bytes[i * 4 + 2] as usize];
        let fourth_sextet = DECODE_TABLE[input_bytes[i * 4 + 3] as usize];

        if first_sextet == INVALID_INDEX
            || second_sextet == INVALID_INDEX
            || third_sextet == INVALID_INDEX
            || fourth_sextet == INVALID_INDEX
        {
            panic!("Invalid base64 string");
        }

        result.push(((first_sextet << 2) | ((second_sextet & 0b110000) >> 4)) as u8);
        result.push((((second_sextet & 0b1111) << 4) | ((third_sextet & 0b111100) >> 2)) as u8);
        result.push((((third_sextet & 0b11) << 6) | fourth_sextet) as u8);
    }

    if let Some(value) = first_extra_value {
        result.push(value);
    }

    if let Some(value) = second_extra_value {
        result.push(value);
    }

    result
}

const ENCODE_TABLE: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

const INVALID_INDEX: u8 = 100;

const DECODE_TABLE: [u8; 128] = [
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    62,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    63,
    52,
    53,
    54,
    55,
    56,
    57,
    58,
    59,
    60,
    61,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    64,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,
    49,
    50,
    51,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
    INVALID_INDEX,
];

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn encode_test() {
        let result = encode(
            "I fairly frequently get asked how to implement a linked list in Rust. The answer honestly depends on what your requirements are, and it's obviously not super easy to answer the question on the spot. As such I've decided to write this book to comprehensively answer the question once and for all."
                .to_string()
                .into_bytes(),
        );
        println!("{}", result);
    }

    #[test]
    fn decode_test() {
        let result = decode("SSBmYWlybHkgZnJlcXVlbnRseSBnZXQgYXNrZWQgaG93IHRvIGltcGxlbWVudCBhIGxpbmtlZCBsaXN0IGluIFJ1c3QuIFRoZSBhbnN3ZXIgaG9uZXN0bHkgZGVwZW5kcyBvbiB3aGF0IHlvdXIgcmVxdWlyZW1lbnRzIGFyZSwgYW5kIGl0J3Mgb2J2aW91c2x5IG5vdCBzdXBlciBlYXN5IHRvIGFuc3dlciB0aGUgcXVlc3Rpb24gb24gdGhlIHNwb3QuIEFzIHN1Y2ggSSd2ZSBkZWNpZGVkIHRvIHdyaXRlIHRoaXMgYm9vayB0byBjb21wcmVoZW5zaXZlbHkgYW5zd2VyIHRoZSBxdWVzdGlvbiBvbmNlIGFuZCBmb3IgYWxsLg==".into());
        println!("{:?}", std::str::from_utf8(&result));
    }

    #[bench]
    fn benchmark(b: &mut Bencher) {
        let bytes = "I fairly frequently get asked how to implement a linked list in Rust. "
            .to_string()
            .into_bytes();

        b.iter(|| encode(bytes.clone()));
    }
}
