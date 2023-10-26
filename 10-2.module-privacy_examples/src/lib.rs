mod outermost {
    pub fn _middle_function() {}

    fn _middle_secret_function() {
        println!("middle_secret_function");
    }

    mod inside {
        pub fn _inner_function() {
            use crate::outermost::_middle_secret_function;
            _middle_secret_function(); // ok
        }

        fn _secret_function() {}
    }
}

pub fn try_me() {
    // outermost::_middle_function(); // ok
    // outermost::_middle_secret_function(); // error
    // outermost::inside::_inner_function(); // error
    // outermost::inside::_secret_function(); // error

    println!("hello world!");
}
