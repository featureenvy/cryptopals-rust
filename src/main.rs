fn main() {
    println!("Hello, world!");
}

pub struct Hex(String);
pub struct Base64(String);

fn hex_to_byte(hex: char) -> u8 {
    match hex {
        '0'...'9' => hex as u8 - '0' as u8,
        'a'...'z' => hex as u8 - ('a' as u8 - 10),
        'A'...'Z' => hex as u8 - ('A' as u8 - 10),
        _ => panic!("{} is not a hex char!", hex)
    }
}

pub fn hex_to_bytes(hex: &Hex) -> Vec<u8> {
    let &Hex(ref raw_input) = hex;
    let mut input_iter = raw_input.chars().map(hex_to_byte);
    let mut res = Vec::new();

    while let (Some(first), Some(second)) = (input_iter.next(), input_iter.next()) {
        res.push(first << 4 | second);
    }

    res
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

pub fn bytes_to_hex(bytes: &[u8]) -> Hex {
    let mut res = String::new();

    for (first, second) in bytes.into_iter().map(byte_to_hex) {
        res.push(first);
        res.push(second);

        println!("{:?} {}", res, first as u8);
    }

    Hex(res)
}

pub fn bytes_to_base64(input: &[u8]) -> Base64 {
    let base64_map = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let to_base64_char = |num, pos| base64_map.chars().nth((num >> pos & 63) as usize).unwrap() as char;

    let mut input_iter = input.iter().map(|&x| x as u32);
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
    let bytes: Vec<u8> = hex_to_bytes(input);
    bytes_to_base64(&bytes)
}

pub fn xor(input: &Hex, key: &Hex) -> Hex {
    let input_b = hex_to_bytes(input);
    let key_b = hex_to_bytes(key);

    let res: Vec<u8> = input_b.into_iter().zip(key_b)
        .map(|(i, k)| i ^ k)
        .collect();

    return bytes_to_hex(&res);
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

        let actual = hex_to_bytes(&Hex("C0FEBabe1337".to_string()));

        assert_eq!(expected, &actual[..]);
    }

    #[test]
    fn transforms_bytes_to_hex() {
        let expected = "C0FEBABE1337";

        let Hex(actual) = bytes_to_hex(&[192 as u8, 254, 186, 190, 19, 55]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn transforms_bytes_to_base64() {
        let expected = "SSdt";

        let Base64(actual) = bytes_to_base64(&vec![73, 39, 109]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn fixed_xor() {
        let expected = "746865206B696420646F6E277420706C6179";

        let Hex(actual) = xor(&Hex("1c0111001f010100061a024b53535009181c".to_string()), &Hex("686974207468652062756c6c277320657965".to_string()));

        assert_eq!(expected, actual);
    }
}
