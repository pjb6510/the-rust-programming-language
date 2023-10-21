struct User {
    name: String,
    _email: String,
    _sign_in_count: u64,
    active: bool,
}

fn main() {
    about_struct();
    about_tuple_struct();

    treat_rectangle();
}

fn about_struct() {
    let user1 = build_user(String::from("someone@example.com"), String::from("Jason"));

    println!("{}", user1.name);

    let user2 = User {
        _email: String::from("another@example.com"),
        name: String::from("anotherusername567"),
        ..user1
    };

    println!("{} {}", user2.name, user2.active);
}

fn build_user(email: String, name: String) -> User {
    return User {
        _email: email,
        name,
        active: true,
        _sign_in_count: 1,
    };
}

fn about_tuple_struct() {
    println!("tuple_struct");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    println!("{}", black.1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn treat_rectangle() {
    println!("getRectangeArea");

    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!(
        "the area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 40,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 45,
        height: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(30);

    println!("square1: {:#?}", square1);
}
