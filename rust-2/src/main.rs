//1 References & Borrowing

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

//2 Slices

// fn first_word(s: &String) -> &str {
//     &s[..1]  // first character
// }

// fn main() {
//     let s = String::from("hello");
//     let first = first_word(&s);
//     println!("First letter: {}", first);
// }

//3 Control Flow

// fn main() {
// let number = 6;

//     if number % 4 == 0 {
//         println!("divisible by 4");
//     } else if number % 3 == 0 {
//         println!("divisible by 3");
//     } else {
//         println!("not divisible by 3 or 4");
//     }
    
// }

//4 Optional Task: Word Length Counter

fn main() {
    let text = "hello world";
    let (word, length) = first_word_and_length(text);

    println!("First word: '{}', length: {}", word, length);
}

fn first_word_and_length(text: &str) -> (&str, usize) {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let word = &text[0..i];
            return (word, word.len());
        }
    }

    // if no space found, return the whole string
    (text, text.len())
}


