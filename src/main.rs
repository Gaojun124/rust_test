use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::create("../test.txt").unwrap();

    // Read the file contents into a string, returns `io::Result<usize>`
    let s = String::from("adfssafasfsdaf");
    for _ in 1..100 {
        f.write(s.as_bytes()).unwrap();
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}