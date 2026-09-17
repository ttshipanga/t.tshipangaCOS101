// Rust program to find roots of a quadratic equation
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("invalid number");

    println!("Enter value for b:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("invalid number");

    println!("Enter value for c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("invalid number");

    let roots: f32 = b * b - 4.0 * a * c;
    println!("Quadratic roots: {}", roots);
    if roots > 0.0 {
        println!("two distinct roots");
    }
    else if roots == 0.0 {
        println!("exactly one real root");
    }
    else if roots < 0.0 {
        println!("no real roots");
    }



    
}
