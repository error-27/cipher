pub fn encode_caesar(input: String, shift: usize) -> String {
    input
    .chars()
    .map(|c| {
        char::from_u32(c as u32 + shift as u32)
    })
    .flatten()
    .collect()
}

pub fn decode_caesar(input: String, shift: usize) -> String {
    input
    .chars()
    .map(|c| {
        char::from_u32(c as u32 - shift as u32)
    })
    .flatten()
    .collect()
}