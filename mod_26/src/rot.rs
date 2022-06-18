pub fn rotate<Int, S>(input: S, shift: Int) -> String
where
    Int: TryInto<u8>
        + core::cmp::Eq
        + core::ops::Rem<Output = Int>
        + core::ops::Add<Output = Int>
        + TryFrom<i32>
        + core::cmp::Ord,
    <Int as TryInto<u8>>::Error: core::fmt::Debug,
    S: AsRef<str>,
{
    let sanitized: u8 = if shift < Int::try_from(0).ok().unwrap() {
        shift % Int::try_from(26).ok().unwrap() + Int::try_from(26).ok().unwrap()
    } else {
        shift % Int::try_from(26).ok().unwrap()
    }
    .try_into()
    .unwrap();

    input
        .as_ref()
        .chars()
        .map(|character| match character {
            'A'..='Z' => (b'A' + (character as u8 - b'A' + sanitized) % 26) as char,
            'a'..='z' => (b'a' + (character as u8 - b'a' + sanitized) % 26) as char,
            _ => character,
        })
        .collect()
}
