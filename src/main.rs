fn main() {
    println!("Hello, world!");
}

pub struct Hex<'a>(&'a str);
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
    let &Hex(raw_input) = hex;
    let mut input_iter = raw_input.chars().map(hex_to_byte);
    let mut res = Vec::new();

    while let (Some(first), Some(second)) = (input_iter.next(), input_iter.next()) {
        res.push(first << 4 | second);
    }

    res
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_from_hex_to_base64() {
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let Base64(actual) = hex_to_base64(&Hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn transforms_hex_to_bytes() {
        let expected = [192, 254, 186, 190, 19, 55];

        let actual = hex_to_bytes(&Hex("C0FEBabe1337"));

        assert_eq!(expected, &actual[..]);
    }

    #[test]
    fn transforms_bytes_to_base64() {
        let expected = "SSdt";

        let Base64(actual) = bytes_to_base64(&vec![73, 39, 109]);

        assert_eq!(actual, expected);
    }
}
