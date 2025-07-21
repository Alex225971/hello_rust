//1 intro to structs
// struct User {
//     username: String,
//     email: String,
//     active: bool
// }

// fn main() {
//     let mut user1 = User {
//         username: String::from("johndoe"),
//         email: String::from("john@example.com"),
//         active: true,
//     };

//     user1.email = String::from("john.doe@newmail.com");
//     user1.active = false;

//     println!("Username: {}", user1.username);
//     println!("Email: {}", user1.email);
//     println!("Active: {}", user1.active);
// }

//2 Tuple and unit structs

// struct RGB(i32, i32, i32);
// struct AlwaysEqual;

// fn main() {
//     let purple = RGB(16,0,16);
//     let subject = AlwaysEqual;

//     println!("Red: {}", purple.0);
//     println!("Green: {}", purple.1);
//     println!("Blue: {}", purple.2);
// }

//3 Enums

//First option is to use derive to print the value as a string
// use std::fmt;

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127,0,0,1);
//     let loopback = IpAddr::V6(String::from("::1"));
//     //:? used to print value
//     println!("Loopback {:?}", loopback);

//     printIpAddr(home);
// }

// //Second option is to implement Display for the custom type

// impl fmt::Display for IpAddr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             IpAddr::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
//             IpAddr::V6(addr) => write!(f, "{}", addr),
//         }
//     }
// }
// fn printIpAddr(addr: IpAddr) {
//     println!("Display implemented IP Address: {}", addr)
// }

//4 pattern matching

struct Person {
    name: String,
    age: u8,
}
enum Status {
    Active,
    Inactive,
    Banned,
}

fn print_status_message(status: &Status) {
    match status {
        Status::Active => println!("User is active."),
        Status::Inactive => println!("User is inactive."),
        Status::Banned => println!("User has been banned."),
    }
}

fn print_person_summary(person: &Person, status: &Status) {
    let status_text = match status {
        Status::Active => "Active",
        Status::Inactive => "Inactive",
        Status::Banned => "Banned",
    };

    println!("{} (age {}) is currently {}", person.name, person.age, status_text);
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let status = Status::Active;

    print_status_message(&status);
    print_person_summary(&person, &status);
}