const TABLE: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

pub fn bytes_to_base64(bytes: Vec<u8>) -> String {
    let mut res = String::from("");

    for i in 0..(bytes.len() / 3) {
        res += TABLE[((bytes[3 * i] & 0b11111100) >> 2) as usize];
        res +=
            TABLE[(((bytes[3 * i] & 0b11) << 4) | ((bytes[3 * i + 1] & 0b11110000) >> 4)) as usize];
        res += TABLE[(((bytes[3 * i + 1] & 0b1111) << 2) | ((bytes[3 * i + 2] & 0b11000000) >> 6))
            as usize];
        res += TABLE[(bytes[3 * i + 2] & 0b111111) as usize];
    }

    let remainder = bytes.len() % 3;
    match remainder {
        0 => return res,
        1 => {
            res += TABLE[((bytes[bytes.len() - 1] & 0b11111100) >> 2) as usize];
            res += TABLE[((bytes[bytes.len() - 1] & 0b11) << 4) as usize];
            res += "==";
            return res;
        }
        2 => {
            res += TABLE[((bytes[bytes.len() - 2] & 0b11111100) >> 2) as usize];
            res += TABLE[(((bytes[bytes.len() - 2] & 0b11) << 4)
                | ((bytes[bytes.len() - 1] & 0b11110000) >> 4)) as usize];
            res += TABLE[((bytes[bytes.len() - 1] & 0b1111) << 2) as usize];
            res += "=";
            return res;
        }
        _ => unreachable!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = bytes_to_base64("I fairly frequently get asked how to implement a linked list in Rust. The answer honestly depends on what your requirements are, and it's obviously not super easy to answer the question on the spot. As such I've decided to write this book to comprehensively answer the question once and for all.".to_string().into_bytes());
        println!("{}", res);
    }
}
