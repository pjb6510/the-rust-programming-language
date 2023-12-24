fn main() {
    let first = "hello";
    let second = String::from("fooBar");

    let result = longest(first, second.as_str());

    println!("{}", result);
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        return first;
    }

    return second;
}
