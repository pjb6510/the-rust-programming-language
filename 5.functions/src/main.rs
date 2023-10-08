fn main() {
    println!("Hello, world!");

    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    let z = {
        let x = 3;
        x + 5
    };

    println!("The value of z is: {z}");

    println!("The value of five is: {}", get_five());
}

fn get_five() -> i32 {
    return 5;
}
