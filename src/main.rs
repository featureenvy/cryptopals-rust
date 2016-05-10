use std::{fmt, convert};

pub struct Hex(String);


impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Hex(ref value) = self;

        write!(f, "Hex({})", value)
    }
}

pub struct Base64(String);

pub struct Bytes(Vec<u8>);

impl Bytes {
    fn len(&self) -> usize {
        let &Bytes(ref value) = self;

        value.len()
    }
}

impl<'a> convert::From<&'a Hex> for Bytes {
    fn from(hex: &Hex) -> Bytes {
        let &Hex(ref raw_input) = hex;
        let mut input_iter = raw_input.chars().map(::hex_to_byte);
        let mut res = Vec::new();

        while let (Some(first), Some(second)) = (input_iter.next(), input_iter.next()) {
            res.push(first << 4 | second);
        }

        Bytes(res)
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Bytes(ref value) = self;

        write!(f, "Bytes({:?})", value)
    }
}

fn hex_to_byte(hex: char) -> u8 {
    match hex {
        '0'...'9' => hex as u8 - '0' as u8,
        'a'...'z' => hex as u8 - ('a' as u8 - 10),
        'A'...'Z' => hex as u8 - ('A' as u8 - 10),
        _ => panic!("{} is not a hex char!", hex)
    }
}

fn byte_to_hex(input: &u8) -> (char, char) {
    let to_char = |byte| match byte {
        i @ 0...9 => '0' as u8 + i,
        i @ _ => 'A' as u8 + (i - 10)
    } as char;

    let first = input >> 4 as u8 & 15;
    let second = input & 15;

    (to_char(first), to_char(second))
}

pub fn bytes_to_hex(bytes: &Bytes) -> Hex {
    let &Bytes(ref raw_bytes) = bytes;
    let mut res = String::new();

    for (first, second) in raw_bytes.into_iter().map(byte_to_hex) {
        res.push(first);
        res.push(second);
    }

    Hex(res)
}

pub fn bytes_to_base64(input: &Bytes) -> Base64 {
    let base64_map = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let to_base64_char = |num, pos| base64_map.chars().nth((num >> pos & 63) as usize).unwrap() as char;

    let &Bytes(ref raw_input) = input;
    let mut input_iter = raw_input.iter().map(|&x| x as u32);
    let mut res = String::new();

    while let (Some(first), Some(second), Some(third)) = (input_iter.next(), input_iter.next(), input_iter.next()) {
        let numeric = first << 16 | second << 8 | third;
        let b64_one = to_base64_char(numeric, 18 as u32);
        let b64_two = to_base64_char(numeric, 12 as u32);
        let b64_three = to_base64_char(numeric, 6 as u32);
        let b64_four = to_base64_char(numeric, 0 as u32);

        res.push(b64_one);
        res.push(b64_two);
        res.push(b64_three);
        res.push(b64_four);
    }

    Base64(res)
}

pub fn hex_to_base64(input: &Hex) -> Base64 {
    let bytes = Bytes::from(input);
    bytes_to_base64(&bytes)
}

pub fn xor(input: &Bytes, key: &Bytes) -> Bytes {
    let &Bytes(ref raw_input) = input;
    let &Bytes(ref raw_key) = key;
    let res: Vec<u8> = raw_input.into_iter().zip(raw_key)
        .map(|(i, k)| i ^ k)
        .collect();

    Bytes(res)
}

pub fn xor_to_hex(input: &Hex, key: &Hex) -> Hex {
    let input_b = Bytes::from(input);
    let key_b = Bytes::from(key);

    let res = xor(&input_b, &key_b);
    return bytes_to_hex(&res);
}

pub fn single_byte_xor(key: char) -> String {
    let input = Bytes::from(&Hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()));
    let key = Bytes(vec![key as u8; input.len()]);
    let result = xor(&input, &key);
    let Bytes(raw_result) = result;

    String::from_utf8(raw_result).unwrap_or("No valid value found.".to_string())
}

pub fn main() {
    for key in b'A'..b'Z'+1 {
        println!("{}: {}", key as char, single_byte_xor(key as char));
    }
    for key in b'a'..b'z'+1 {
        println!("{}: {}", key as char, single_byte_xor(key as char));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_from_hex_to_base64() {
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let Base64(actual) = hex_to_base64(&Hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()));
        assert_eq!(expected, actual);
    }

    #[test]
    fn transforms_hex_to_bytes() {
        let expected = [192, 254, 186, 190, 19, 55];

        let Bytes(actual) = Bytes::from(&Hex("C0FEBabe1337".to_string()));

        assert_eq!(expected, &actual[..]);
    }

    #[test]
    fn transforms_bytes_to_hex() {
        let expected = "C0FEBABE1337";

        let Hex(actual) = bytes_to_hex(&Bytes(vec![192 as u8, 254, 186, 190, 19, 55]));

        assert_eq!(expected, actual);
    }

    #[test]
    fn transforms_bytes_to_base64() {
        let expected = "SSdt";

        let Base64(actual) = bytes_to_base64(&Bytes(vec![73, 39, 109]));

        assert_eq!(actual, expected);
    }

    #[test]
    fn fixed_xor() {
        let expected = "746865206B696420646F6E277420706C6179";

        let Hex(actual) = xor_to_hex(&Hex("1c0111001f010100061a024b53535009181c".to_string()), &Hex("686974207468652062756c6c277320657965".to_string()));

        assert_eq!(expected, actual);
    }
}
