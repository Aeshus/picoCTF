fn main() {}

#[cfg(test)]
mod tests {
    use mod_26::rot::rotate;

    #[test]
    fn rot_0() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(input, 0), output);
    }

    #[test]
    fn rot_3() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "defghijklmnopqrstuvwxyzabc".to_string();
        assert_eq!(rotate(input, 3), output);
    }

    #[test]
    fn rot_13() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "nopqrstuvwxyzabcdefghijklm".to_string();
        assert_eq!(rotate(input, 13), output);
    }

    #[test]
    fn rot_22() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "wxyzabcdefghijklmnopqrstuv".to_string();
        assert_eq!(rotate(input, 22), output);
    }

    #[test]
    fn rot_26() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(input, 26), output);
    }

    #[test]
    fn rot_34() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "ijklmnopqrstuvwxyzabcdefgh".to_string();
        assert_eq!(rotate(input, 34), output);
    }

    #[test]
    fn rot_200() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "stuvwxyzabcdefghijklmnopqr".to_string();
        assert_eq!(rotate(input, 200), output);
    }
}
