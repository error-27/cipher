pub fn encode_caesar(input: String, shift: usize) -> String {
    input
    .chars()
    .map(|c| {
        if !c.is_alphabetic() {
            return Some(c)
        }
        if c.is_lowercase() && c as u32 + shift as u32 > 'z' as u32 {
            let remainder = shift as u32 - ('z' as u32 - c as u32);
            return char::from_u32('a' as u32 - 1 + remainder)
        }
        if c.is_uppercase() && c as u32 + shift as u32 > 'Z' as u32 {
            let remainder = shift as u32 - ('Z' as u32 - c as u32);
            return char::from_u32('A' as u32 - 1 + remainder)
        }
        char::from_u32(c as u32 + shift as u32)
    })
    .flatten()
    .collect()
}

pub fn decode_caesar(input: String, shift: usize) -> String {
    encode_caesar(input, 26 - shift)
}