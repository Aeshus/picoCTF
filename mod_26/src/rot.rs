pub fn rotate(input: String, shift: u8) -> String {
    let mut output = String::new();

    for character in input.chars() {
        let case = match character {
            'A'..='Z' => b'A',
            'a'..='z' => b'a',
            _ => {
                output.push(character);
                continue;
            }
        };

        let output_char = (case + (character as u8 - case + shift) % 26) as char;

        output.push(output_char);
    }

    output
}
