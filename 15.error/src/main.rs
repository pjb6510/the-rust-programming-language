#![allow(unused)]

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    let username = read_username_from_file().expect("failed to read username");
    println!("{}", username);
}

fn open_hello_txt() -> File {
    return match File::open("./hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("./hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem {:?}", e)
                }
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("hello.txt");

    let mut file = match file {
        Ok(unwrapped_file) => unwrapped_file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    return match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s);

    return Ok(s);
}
