use std::{fmt, convert, ops};
use Hex;

pub struct Bytes {
    pub value: Vec<u8>
}

impl Bytes {
    pub fn len(&self) -> usize {
        self.value.len()
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

        Bytes {value: res}
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bytes({:?})", self.value)
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = ::std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.value.deref()
    }
}
