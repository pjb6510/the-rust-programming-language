fn main() {
    show_variables();
    show_numbers();
    show_booleans();
    show_chars();

    show_tuple();
    show_array();
}

fn show_variables() {
    println!("show variables");

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("{MAX_POINTS}");
    println!("\n");
}

fn show_numbers() {
    println!("show numbers");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "add:{}, sub:{}, multp:{}, div:{}, rmndr:{}",
        sum, difference, product, quotient, remainder
    );

    println!("\n");
}

fn show_booleans() {
    println!("show booleans");

    let t = true;
    let f: bool = false;

    println!("true:{}, false:{}", t, f);

    println!("\n");
}

fn show_chars() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    println!("\n");
}

fn show_tuple() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    let pos = tup;

    println!("The value of y is {}", y);

    println!("The value of x is {}", pos.0);

    println!("\n");
}

fn show_array() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("{} {}", first, second);
    println!("\n");
}
