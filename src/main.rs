extern crate regex;

use std::fmt;
use std::collections::HashMap;

use regex::Regex;

mod bytes;
use bytes::Bytes;

pub struct Hex(String);


impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Hex(ref value) = self;

        write!(f, "Hex({})", value)
    }
}

pub struct Base64(String);


fn hex_to_byte(hex: char) -> u8 {
    match hex {
        '0'...'9' => hex as u8 - '0' as u8,
        'a'...'z' => hex as u8 - ('a' as u8 - 10),
        'A'...'Z' => hex as u8 - ('A' as u8 - 10),
        _ => panic!("{} is not a hex char!", hex)
    }
}

fn byte_to_hex(input: u8) -> (char, char) {
    let to_char = |byte| match byte {
        i @ 0...9 => '0' as u8 + i,
        i @ _ => 'A' as u8 + (i - 10)
    } as char;

    let first = input >> 4 as u8 & 15;
    let second = input & 15;

    (to_char(first), to_char(second))
}

pub fn bytes_to_hex(bytes: &Bytes) -> Hex {
    let mut res = String::new();

    for (first, second) in bytes.iter().map(|c: &u8| byte_to_hex(*c)) {
        res.push(first);
        res.push(second);
    }

    Hex(res)
}

pub fn bytes_to_base64(input: &Bytes) -> Base64 {
    let base64_map = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let to_base64_char = |num, pos| base64_map.chars().nth((num >> pos & 63) as usize).unwrap() as char;

    let mut input_iter = input.iter().map(|x| *x as u32);
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
    let res: Vec<u8> = input.iter().zip(key.iter())
        .map(|(i, k)| i ^ k)
        .collect();

    Bytes{value: res}
}

pub fn xor_to_hex(input: &Hex, key: &Hex) -> Hex {
    let input_b = Bytes::from(input);
    let key_b = Bytes::from(key);

    let res = xor(&input_b, &key_b);
    return bytes_to_hex(&res);
}

pub fn single_byte_xor(input: &Bytes, key: char) -> String {
    let key = Bytes{value: vec![key as u8; input.len()]};
    let result = xor(&input, &key);

    String::from_utf8(result.value).unwrap_or("No valid value fond.".to_string())
}

fn count_letters(value: &str) -> usize {
    value.chars().filter(|c| c.is_alphabetic())
        .count()
}

pub fn crack_single_byte_xor(input: Bytes) -> String {
    let ascii_checker = Regex::new(r"^[:print:]*$").unwrap(); // unwrap because otherwise it is a bug!

    let possible_values: HashMap<u32, String> = (b'A'..b'Z'+1).chain(b'a'..b'z'+1)
        .map(|key| single_byte_xor(&input, key as char))
        .filter(|decrypt| ascii_checker.is_match(decrypt))
        .fold(HashMap::new(), |mut map, decrypt| { map.insert((count_letters(&decrypt) as f32 / decrypt.len() as f32 * 100 as f32) as u32, decrypt.clone()); map });

    let max = possible_values.keys().max().expect("No value found.");
    possible_values.get(max).expect("WAT???").clone()
}

pub fn main() {
    // for key in b'A'..b'Z'+1 {
    //     println!("{}: {}", key as char, single_byte_xor(key as char));
    // }
    // for key in b'a'..b'z'+1 {
    //     println!("{}: {}", key as char, single_byte_xor(key as char));
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn transforms_from_hex_to_base64() {
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let Base64(actual) = hex_to_base64(&Hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()));
        assert_eq!(expected, actual);
    }

    #[test]
    fn transforms_hex_to_bytes() {
        let expected = [192, 254, 186, 190, 19, 55];

        let actual = Bytes::from(&Hex("C0FEBabe1337".to_string()));

        assert_eq!(expected, &actual[..]);
    }

    #[test]
    fn transforms_bytes_to_hex() {
        let expected = "C0FEBABE1337";

        let Hex(actual) = bytes_to_hex(&Bytes{value: vec![192 as u8, 254, 186, 190, 19, 55]});

        assert_eq!(expected, actual);
    }

    #[test]
    fn transforms_bytes_to_base64() {
        let expected = "SSdt";

        let Base64(actual) = bytes_to_base64(&Bytes{value: vec![73, 39, 109]});

        assert_eq!(actual, expected);
    }

    #[test]
    fn fixed_xor() {
        let expected = "746865206B696420646F6E277420706C6179";

        let Hex(actual) = xor_to_hex(&Hex("1c0111001f010100061a024b53535009181c".to_string()), &Hex("686974207468652062756c6c277320657965".to_string()));
 
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_crack_single_byte_xor() {
        let input = Bytes::from(&Hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()));

        assert_eq!("Cooking MC's like a pound of bacon", crack_single_byte_xor(input));
    }
}
