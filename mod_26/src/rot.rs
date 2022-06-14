pub fn rotate<T>(input: String, shift: T) -> String
where
    T: TryInto<u8>
        + Eq
        + core::ops::Rem<Output = T>
        + core::ops::Add<Output = T>
        + TryFrom<i32>
        + core::fmt::Debug
        + std::cmp::Ord
        + core::cmp::PartialEq,
    <T as TryInto<u8>>::Error: core::fmt::Debug,
{
    let mut output = String::new();

    let mut sanitized: T = shift % T::try_from(26).ok().unwrap();

    if sanitized < T::try_from(0).ok().unwrap() {
        sanitized = sanitized + T::try_from(26).ok().unwrap()
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
