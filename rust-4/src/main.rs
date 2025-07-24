//practise simple user input
use std::io;

struct Rectangle {
    height: u32,
    width: u32,
}

//simple constructor for a custom struct
impl Rectangle {
    fn new (width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn bool_to_yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("The area of rect1 is {} square pixels.", rect1.area());

    println!(
        "Can rect1 hold rect2? {}",
        bool_to_yes_no(rect1.can_hold(&rect2))
    );
    println!(
        "Can rect1 hold rect3? {}",
        bool_to_yes_no(rect1.can_hold(&rect3))
    );

    // print_name();
}

// fn print_name() {
//     println!("Please enter your name:");

//     let mut user_input = String::new();

//     io::stdin()
//         .read_line(&mut user_input)
//         .expect("Failed to read line");

//     let trimmed_input = user_input.trim();

//     println!("Hello, {}!", trimmed_input);
// }
