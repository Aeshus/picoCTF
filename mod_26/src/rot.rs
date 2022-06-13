pub fn rotate(input: String, shift: usize) -> String {
    let mut output = String::new();

    let sanitized_shift: u8 = (shift % 26).try_into().unwrap();

    for character in input.chars() {
        let case = match character {
            'A'..='Z' => b'A',
            'a'..='z' => b'a',
            _ => {
                output.push(character);
                continue;
            }
        };

        let output_char = (case + (character as u8 - case + sanitized_shift) % 26) as char;

        output.push(output_char);
    }

    output
}

pub fn decode(input: String, shift: usize) -> String {
    rotate(input, 26 - (shift % 26))
}

pub fn encode(input: String, shift: usize) -> String {
    rotate(input, shift)
}
