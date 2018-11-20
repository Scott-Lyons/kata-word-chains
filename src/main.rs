//use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

static mut vec: Vec<&str> = Vec::new();

fn main() {
    let filename = "c:\\git\\kata-word-chains\\src\\wordlist.txt";

    println!("In file {}", filename);

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut split = contents.lines();

    //let vec = split.collect::<Vec<&str>>();
    vec = split.collect();
    
    if vec.contains(&"blhblahblah") {
        println!("Yes")
    } else {
        println!("No")
    }

    //println!("With text:\n{}", contents);
}

fn word_exist() -> bool {
    return true;
}

