#![allow(unused)]

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("hello modules!");
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum MouseEvent {
    Down,
    Up,
    Enter,
    Leave,
}

use a::series::of;
use MouseEvent::*;
use TrafficLight::{Red, Yellow};

fn main() {
    of::nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    let mousedown = Down;
}
