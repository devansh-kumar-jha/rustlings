// if1.rs

use std::io;

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `rustlings hint if1` for hints
    if a>=b { a }
    else { b }
}

fn main() {
    
    let mut a1= String::new();
    let mut b1= String::new();
    
    println!("Enter first number : ");
    io::stdin()
        .read_line(&mut a1)
        .expect("Not correct input !!!");
    println!("Enter second number : ");
    io::stdin()
        .read_line(&mut b1)
        .expect("Not correct input !!!");    
    
    let a = a1.trim();
    match a.parse::<i32>() {
        Err(..) => println!("this was not an integer: {}", a),
        Ok(a) => {
            let b = b1.trim();
            match b.parse::<i32>() {
                Err(..) => println!("this was not an integer: {}",b),
                Ok(b) => println!("Larger of the numbers : {}",bigger(a,b)),
            };
        },
    };    
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
