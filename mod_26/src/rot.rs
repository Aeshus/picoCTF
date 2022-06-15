pub fn rotate<Int, S>(input: S, shift: Int) -> String
where
    Int: TryInto<u8>
        + core::cmp::Eq
        + core::ops::Rem<Output = Int>
        + core::ops::Add<Output = Int>
        + TryFrom<i32>
        + core::cmp::Ord,
    <Int as TryInto<u8>>::Error: core::fmt::Debug,
    S: Into<String>,
{
    let input: String = input.into();

    let mut output = String::new();

    let mut sanitized: Int = shift % Int::try_from(26).ok().unwrap();

    if sanitized < Int::try_from(0).ok().unwrap() {
        sanitized = sanitized + Int::try_from(26).ok().unwrap()
    }

    let sanitized: u8 = sanitized.try_into().unwrap();

    for character in input.chars() {
        let case = match character {
            'A'..='Z' => b'A',
            'a'..='z' => b'a',
            _ => {
                output.push(character);
                continue;
            }
        };

        let output_char = (case + (character as u8 - case + sanitized) % 26) as char;

        output.push(output_char);
    }

    output
}
