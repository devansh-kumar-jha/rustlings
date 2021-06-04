// move_semantics5.rs
// Make me compile without adding, removing, or changing any of the
// lines in `main()`.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;                                // This requires that y is owned by main but 
    let z = &mut *y;                          // after let z = &mut *y  we cannot use *y  as it is borrowed
    *z += 1000;
    assert_eq!(x, 1200);
}
