#![allow(unused)]

#[derive(Debug)]
enum Position {
    Top,
    Jungle,
    Mid,
    AdCarry,
    Support,
}

#[derive(Debug)]
struct Person {
    name: String,
    position: Position,
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    for i in &mut vector {
        *i += 50;
    }

    let num = &mut vector[0];
    *num += 10;

    println!("vector: {:?}", vector);

    let doran = Person {
        name: String::from("Doran"),
        position: Position::Top,
    };
    let peanut = Person {
        name: String::from("Peanut"),
        position: Position::Jungle,
    };
    let chovy = Person {
        name: String::from("Chovy"),
        position: Position::Mid,
    };
    let peyz = Person {
        name: String::from("Peyz"),
        position: Position::AdCarry,
    };
    let delight = Person {
        name: String::from("Delight"),
        position: Position::Support,
    };

    let gen_g = vec![doran, peanut, chovy, peyz, delight];

    for member in &gen_g {
        println!("{:?}", member);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        println!("{:?}", cell);
    }
}
