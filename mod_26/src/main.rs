use mod_26::rot::rotate;
use std::env::args;

fn main() {
    for arg in args().skip(1) {
        println!("{} -> {}", arg, rotate(arg.clone(), 13));
    }
}

#[cfg(test)]
mod tests {
    use mod_26::rot::rotate;

    #[test]
    fn encode_0() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(rotate(input, 0), output);
    }

    #[test]
    fn encode_3() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "defghijklmnopqrstuvwxyzabc";
        assert_eq!(rotate(input, 3), output);
    }

    #[test]
    fn encode_13() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "nopqrstuvwxyzabcdefghijklm";
        assert_eq!(rotate(input, 13), output);
    }

    #[test]
    fn encode_22() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "wxyzabcdefghijklmnopqrstuv";
        assert_eq!(rotate(input, 22), output);
    }

    #[test]
    fn encode_26() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(rotate(input, 26), output);
    }

    #[test]
    fn encode_34() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "ijklmnopqrstuvwxyzabcdefgh";
        assert_eq!(rotate(input, 34), output);
    }

    #[test]
    fn encode_200() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = "stuvwxyzabcdefghijklmnopqr";
        assert_eq!(rotate(input, 200), output);
    }

    #[test]
    fn rot_0() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(rotate(rotate(input.clone(), 0), 0), input);
    }
    #[test]
    fn rot_255() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(rotate(rotate(input.clone(), 255), -255), input);
    }

    #[test]
    fn rot_512() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(rotate(rotate(input.clone(), 512), -512), input);
    }

    #[test]
    fn strange_rot() {
        let input = "ABCDEFgsdkfjldskjklsdjgklds";
        assert_eq!(
            rotate(input.clone(), 10),
            rotate(rotate(input.clone(), 12), -2)
        );
    }
}
