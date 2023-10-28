#![allow(unused)]

fn main() {
    extend_string();
    index_string();
}

fn create_new_string() {
    let mut s = String::new();

    let some_str = "initial contents";
    let s = some_str.to_string();

    // same
    let s = "initial contents".to_string();
}

fn hello_utf8() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn extend_string() {
    let mut s = "foo".to_string();
    s.push_str("bar");

    let mut s = "lo".to_string();
    s.push('l');

    let s1 = "hello ".to_string();
    let s2 = "world".to_string();

    // s1's ownership is moved
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", &s1, &s2, &s3);

    println!("s: {}", s);
}

fn index_string() {
    let s1 = String::from("hello");

    // error;
    // let h = s1[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("s: {}", s);

    let hello = "안녕하세요";

    for byte in hello.bytes() {
        println!("{}", byte);
    }

    for byte in hello.chars() {
        println!("{}", byte);
    }
}
