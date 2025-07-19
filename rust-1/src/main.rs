fn main() {
    ownership_demo();
}

fn ownership_demo() {
    let s1 = String::from("hello");
    let s2 = " world";

    let s3 = format!("{}{}", s1, s2);

    println!("s3: {}", s3);
    
    // s1 is still usable
    let s4 = format!("{} again!", s1);
    println!("s4: {}", s4);
}