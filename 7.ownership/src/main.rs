fn main() {
    show_ownership();
    show_borrowing();
    show_slices();
}

fn show_ownership() {
    println!("show_ownership");
    let s1 = gives_ownership();

    let _s2 = String::from("hello");

    let s3 = takes_and_gives_back(s1);

    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn show_borrowing() {
    println!("show_borrowing");

    let s = String::from("world");

    let length = calc_length(&s);

    println!("length {length}");

    let mut s2 = String::from("hello");

    change_string(&mut s2);

    println!("s2 {s2}");
}

fn calc_length(s: &String) -> usize {
    return s.len();
}

fn change_string(s: &mut String) {
    s.push_str("foobar");
}

fn show_slices() {
    println!("show_slices");

    let s = String::from("Hello World");

    let hello = first_word(&s);

    println!("{hello}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
