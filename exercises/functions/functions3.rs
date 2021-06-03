// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// I AM NOT DONE

fn main() {
    let x=call_me(4);
    println!("Number passed is : {}",x);    
}

fn call_me(num: u32)->i32 {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
    let b = num as i32;
    b
}
