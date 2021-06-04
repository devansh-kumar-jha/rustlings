// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute the command `rustlings hint if2` if you want a hint :)

// I AM NOT DONE

use std::io;

pub fn fizz_if_foo(fizzish: &str) -> &str {
    if fizzish=="fizz\n" { "foo" }                   // \n is used presuming that the user presses enter to give the input 
                                                     // code was checked on VS code where input is given side by side :)
    else if fizzish=="foo\n" { "fizz" }
    else { fizzish }
}

fn main() {
    let mut input = String::new();
    println!("Enter the string");
    io::stdin()
        .read_line(&mut input)
        .expect("Not correct input :( !!!"); 
    
    println!("Result : {}",fizz_if_foo(&input));    
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(fizz_if_foo("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(fizz_if_foo("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(fizz_if_foo("literally anything"), "baz")
    }
}
