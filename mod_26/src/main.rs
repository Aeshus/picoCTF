use mod_26::rot::rotate;
use std::env::args;

fn main() {
    for arg in args().skip(1) {
        println!("{} -> {}", arg, rotate(arg.clone(), 255));
    }
}

#[cfg(test)]
mod tests {
    use mod_26::rot::rotate;

    #[test]
    fn encode_0() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(input, 0), output);
    }

    #[test]
    fn encode_3() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "defghijklmnopqrstuvwxyzabc".to_string();
        assert_eq!(rotate(input, 3), output);
    }

    #[test]
    fn encode_13() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "nopqrstuvwxyzabcdefghijklm".to_string();
        assert_eq!(rotate(input, 13), output);
    }

    #[test]
    fn encode_22() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "wxyzabcdefghijklmnopqrstuv".to_string();
        assert_eq!(rotate(input, 22), output);
    }

    #[test]
    fn encode_26() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(input, 26), output);
    }

    #[test]
    fn encode_34() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "ijklmnopqrstuvwxyzabcdefgh".to_string();
        assert_eq!(rotate(input, 34), output);
    }

    #[test]
    fn encode_200() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        let output = "stuvwxyzabcdefghijklmnopqr".to_string();
        assert_eq!(rotate(input, 200), output);
    }

    #[test]
    fn rot_0() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(rotate(input.clone(), 0), 0), input);
    }
    #[test]
    fn rot_255() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(rotate(input.clone(), 255), -255), input);
    }

    #[test]
    fn rot_512() {
        let input = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert_eq!(rotate(rotate(input.clone(), 512), -512), input);
    }

    #[test]
    fn strange_rot() {
        let input = "ABCDEFgsdkfjldskjklsdjgklds".to_string();
        assert_eq!(
            rotate(input.clone(), 10),
            rotate(rotate(input.clone(), 12), -2)
        );
    }
}
