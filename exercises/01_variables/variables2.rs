// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut x: u8;
    
    x = 5;
    if x == 10 {
        println!("x is ten!");
        x += 1;
        println!("x +1 is {}!", x);
    } else {
        println!("x is not ten! {}", x);
    }
}
