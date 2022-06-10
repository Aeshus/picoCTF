# PicoCTF - Obedient Cat

## Problem:
- This file has a flag in plain sight (aka "in-the-clear"). 

## Hints:
1. Any hints about entering a command into the Terminal (such as the next one), will start with a '$'... everything after the dollar sign will be typed (or copy and pasted) into your Terminal.
  - PicoCTF uses the common notation of `$` to prepend any shell command.
  - This also comes with the meaning that the command should be run as a regular user, and a `#` would be used if it was to be run as root (or administrator). [0]
2. To get the file accessible in your shell, enter the following in the Terminal prompt: $ wget (site).
  - Picoctf suggests to use Wget to download content from it's remote servers.
  - It also comes with many useful features to crawl websites and download all referenced files. [1]
  - For most circumstances though, you will be fine just downloading it through your web browser.
3. $ man cat
  - "Man" is the unix command for manual pages, so this command brings up the documentation for the cat command. [2]
  - The description given for the cat command is "concatenate files and prnt on the standard output". [3]
  - Cat is commonly used for just reading simple files to the terminal (STDOUT).

## Solution:
In an attempt to overcomplicate this, we will be building an command-line-argument parser and will manually read and interpret the file as bytes.

First and for most, we need to import all the modules we will be using.
```rust
use std::fs;
use std::path::{PathBuf};
use std::env::{args, Args};
```

First thing we need to do is get all the arguments passed to the CLI, so we will use Rust's Args functions.
Just like how hint number 3 says `$ man cat`, we want to be able to run `$ obedient_cat ~/file`. 
This means we will be expecting 2 inputs, the name of the program (0), and the path to the file (1).

To express this relationship, it would make sense for us to build a struct.
```rust
struct Cli {
  program_name: Option<String>,
  path_to_file: Option<String>
}
```

Next, we need to be able to build the CLI, and to do that, we can impliemnt the From trait for the CLI.
Rust's command line arguments are accessed through the args() function, which returns an interator, type Args, over a group of strings.
The result would look something like this:
```rust
impl From<Args> for Cli {
    fn from(mut args: Args) -> Self {
        let program_name = args.next();
        let path_to_file = args.next();

        if args.next() != None {
            println!("Do not supply more than 2 command line arguments");
        }

        Self {
            program_name,
            path_to_file
        }
    }
}
```

The next function, when used on an iterator, returns Option<String> for the current element and moves the cursor to the next element.
Option is Rust's analogue for many language's Nil or Null. An Option is either `None` or `Some(T)`. In this circumstance, if the iterator isn't empty, it will return the command line argument.

We do that twise, as the Args includes the calling of the original function, and then we need to collect the path.
There is no need to worry about extra command line arguments being input, so we can just print out a warning and not panic the whole program.

Now, lets build the main function.
```rust
fn main() {
  let cli = Cli::from(args());
  
  if cli.program_name == None { panic!("Program Name Not Found" ); }
  if cli.path_to_file == None { panic!("Path To File Not Found" ); }
  
  let path = PathBuf::from(cli.path_to_file.unwrap())
    .canonicalize()
    .expect("Path supplied is not a path");
    
  println!("{:?}", read_file(path));  
}
```

What we do here is first call our from function to build our CLI.
Then we do some quick sanity checks to verify that the arguments were parsed correctly and were actually given.

We then create the PathBuffer, using the cli's path as it's input and then canonicalizing it.
Canonicalizing just returns the absolute and completely resolved path, making it significantly easier to handle. [#]

Then, we just plug that into our next function, read_file, and print it out!

```rust
fn read_file(path: Pathbuf) -> String {
  fs::read_to_string(path).expect("Couldn't read file");
}
```

While you could theoretically inline this function, it makes it much cleaner for the tests and calls to it.
All it does is read the file of the path and then return it as a String, as simple as that.

Finally, the tests.
```rust
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
```
  
Rust has a very powerful testing system, so I want to use this here.
While we don't *really* need to use tests, as all we're testing is that Rust's internal standard library is working, but that's just the nature of this very simple CTF challenge.
Now you just need to run `$ cargo run (file)` and you have the flag!

## References
[0] - https://stackoverflow.com/a/48215530
[1] - https://www.gnu.org/software/wget/
[2] - https://en.wikipedia.org/wiki/Man_page
[3] - https://man7.org/linux/man-pages/man1/cat.1.html
