#[derive(Debug)]
enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn show(&self) {
        println!("show! {:#?}", self);
    }
}

enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quater(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
    // ... etc
}

fn main() {
    show_messages();
    show_options();

    show_coin();
}

fn show_messages() {
    let quit_message = Message::Quit;
    let write_message = Message::Write(String::from("hello"));
    let change_color_message = Message::ChangeColor(0, 0, 0);
    let move_message = Message::Move { _x: 32, _y: 14 };

    show_message(move_message);
    show_message(write_message);
    show_message(change_color_message);
    show_message(quit_message);
}

fn show_message(message: Message) {
    message.show()
}

fn show_options() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();

    println!("sum: {sum}");

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        Some(i) => Some(i + 1),
        None => None,
    };
}

fn show_coin() {
    let first_coin = Coin::Penny;
    let second_coin = Coin::Quater(UsState::Alabama);

    let first_coin_value = value_in_cents(first_coin);
    let second_coin_value = value_in_cents(second_coin);
    println!("first_coin: {first_coin_value}");
    println!("second_coin: {second_coin_value}");
}

fn value_in_cents(coin: Coin) -> u32 {
    return match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return 1;
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        }
    };
}

fn _handle_u8(num: u8) {
    match num {
        3 => println!("three"),
        _ => (),
    };
}

fn _show_if_let() {
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three")
    }
}
