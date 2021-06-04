// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    // let vec0 : Vec<i32> = Vec::new();

    // let vec2 : Vec<i32> = Vec::new();                                              // METHOD 1
    // let mut vec1 = fill_vec(vec2);

    // let mut vec0 : Vec<i32> = Vec::new();
    // let mut vec1 = fill_vec(&mut vec0);                                                     // METHOD 2
 
    let vec0 : Vec<i32> = Vec::new();
    let mut vec1 = fill_vec(&vec0);                                                    // METHOD 3

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {                                              //  METHOD 1
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {                                                      // METHOD 2
//     let vec : &mut Vec<i32> = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     (*vec).to_vec()
// }

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {                                              //  METHOD 3
    let mut vec : Vec<i32> = vec.to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}