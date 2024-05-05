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

pub fn encode_vigenere(input: String, key: String) -> String {
    let mut i = 0;
    let mut output = "".to_string();
    for c in input.chars() {
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let shift = key_char.to_ascii_lowercase() as u32 - ('a' as u32);

        if !c.is_alphabetic() {
            output.push(c);
            continue;
        }
        if c.is_lowercase() && c as u32 + shift as u32 > 'z' as u32 {
            let remainder = shift as u32 - ('z' as u32 - c as u32);
            output.push(char::from_u32('a' as u32 - 1 + remainder).unwrap());
            i += 1;
            continue;
        }
        if c.is_uppercase() && c as u32 + shift as u32 > 'Z' as u32 {
            let remainder = shift as u32 - ('Z' as u32 - c as u32);
            output.push(char::from_u32('A' as u32 - 1 + remainder).unwrap());
            i += 1;
            continue
        }
        output.push(char::from_u32(c as u32 + shift as u32).unwrap());
        i += 1;
    }
    output
}

pub fn decode_vigenere(input: String, key: String) -> String {
    let mut i = 0;
    let mut output = "".to_string();
    for c in input.chars() {
        let key_char = key.chars().nth(i % key.len()).unwrap();
        let shift = 26 - (key_char.to_ascii_lowercase() as u32 - ('a' as u32));

        if !c.is_alphabetic() {
            output.push(c);
            continue;
        }
        if c.is_lowercase() && c as u32 + shift as u32 > 'z' as u32 {
            let remainder = shift as u32 - ('z' as u32 - c as u32);
            output.push(char::from_u32('a' as u32 - 1 + remainder).unwrap());
            i += 1;
            continue;
        }
        if c.is_uppercase() && c as u32 + shift as u32 > 'Z' as u32 {
            let remainder = shift as u32 - ('Z' as u32 - c as u32);
            output.push(char::from_u32('A' as u32 - 1 + remainder).unwrap());
            i += 1;
            continue
        }
        output.push(char::from_u32(c as u32 + shift as u32).unwrap());
        i += 1;
    }
    output
}