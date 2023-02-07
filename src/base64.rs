//RFC 4648
const TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const PADDING: char = '=';

pub fn base64_encode(buffer: &[u8]) -> String {
    let in_len = buffer.len();
    let out_len = (in_len + 2) / 3 * 4;
    let mut output = String::with_capacity(out_len);

    for i in 0..in_len / 3 {
        encode_slice(&buffer[i*3..=i*3+2], &mut output);
    }
    // padding
    if in_len % 3 != 0 {
        let padding_in:[u8;4] = [buffer[in_len]];
    }
    assert_eq!(out_len, output.len());

    output
}

pub fn base64_decode(s: &str) -> Vec<u8> {
    todo!()
}

/// panic if slice length isn't 3
fn encode_slice(slice:&[u8],str:&mut String){
    let a = slice[0];
        let b = slice[1];
        let c = slice[2];
        let char1 = a >> 2;
        let char2 = (a << 4 | b >> 4) & 0b00111111;
        let char3 = (b << 2 | c >> 6) & 0b00111111;
        let char4 = c & 0b00111111;
        // TODO use ascii to prevent type casting or raw pointer?
        str.push(TABLE[char1 as usize]);
        str.push(TABLE[char2 as usize]);
        str.push(TABLE[char3 as usize]);
        str.push(TABLE[char4 as usize]);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(base64_encode(b"abc"), "YWJj");
        assert_eq!(base64_encode(b"gsgcbirmsdkgmer"), "Z3NnY2Jpcm1zZGtnbWVy")
    }
    #[test]
    fn padding() {
        assert_eq!(base64_encode(b"abcd"), "YWJjZA==");
        assert_eq!(base64_encode(b"abcde"), "YWJjZGU=");
    }
}
