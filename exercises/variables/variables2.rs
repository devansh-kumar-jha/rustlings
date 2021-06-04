// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// I AM NOT DONE

use std::io;

fn main() {
    let mut x=String::new();
    
    println!("Enter your number");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line :( !!!");

    if &x == "10\n" {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
