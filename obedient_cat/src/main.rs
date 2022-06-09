use std::env::{args, Args};
use std::fs;
use std::path::PathBuf;

struct Cli {
    program_name: Option<String>,
    path_to_file: Option<String>,
}

impl From<Args> for Cli {
    fn from(mut args: Args) -> Self {
        let program_name = args.next();
        let path_to_file = args.next();

        if args.next() != None {
            println!("Do not supply more than 2 command line arguments");
        }

        Self {
            program_name,
            path_to_file,
        }
    }
}

fn main() {
    let cli = Cli::from(args());

    if cli.program_name == None {
        panic!("Program Name Not Found")
    };
    if cli.path_to_file == None {
        panic!("Path To File Not Found")
    };

    let path = PathBuf::from(cli.path_to_file.unwrap())
        .canonicalize()
        .expect("Path supplied is not a path");

    println!("{:?}", read_file(path));
}

fn read_file(path: PathBuf) -> String {
    fs::read_to_string(path).expect("Couldn't read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_flag_1() {
        let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/example1");
        assert_eq!(read_file(path), "picoCTF{FakeFlag1}".to_string())
    }

    #[test]
    fn example_flag_2() {
        let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/example2");
        assert_eq!(read_file(path), "{Testing,Testing}".to_string());
    }

    #[test]
    fn example_flag_3() {
        let mut path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/example3");
        assert_eq!(read_file(path), "{';asd\\n\\t/}".to_string());
    }
}
