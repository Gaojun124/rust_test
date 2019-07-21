use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod mods;
use mods::test;
fn main() {
    let mut f = File::create("../test.txt").unwrap();

    // Read the file contents into a string, returns `io::Result<usize>`
    let s = String::from("adfssafasfsdaf");
    for _ in 1..100 {
        f.write(s.as_bytes()).unwrap();
    }
    println!("{:?}", Path::new("../test.txt").file_stem().unwrap());

    test::run();

    mods::run();

    // `file` goes out of scope, and the "hello.txt" file gets closed
}