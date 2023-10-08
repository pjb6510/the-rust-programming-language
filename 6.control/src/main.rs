fn main() {
    smaller_than_five(3);
    smaller_than_five(6);

    assign_with_if();

    run_loop();
    run_while_loop();
    run_for_loop();
}

fn smaller_than_five(number: i32) {
    println!("input number is {}", number);

    if number < 5 {
        println!("true");
    } else {
        println!("false")
    }
}

fn assign_with_if() {
    let condition = false;

    let number = if condition { 5 } else { 6 };

    println!("assigned number is {}", number)
}

fn run_loop() {
    let mut count = 0;

    loop {
        println!("again!");

        count += 1;

        if count == 3 {
            break;
        }
    }
}

fn run_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn run_for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
