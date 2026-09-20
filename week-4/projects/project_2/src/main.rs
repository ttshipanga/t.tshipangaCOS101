//Rust program to calculate employee's incentives

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter years of experience");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience:i32= input1.trim().parse().expect("Not a valid input");
    
    if experience < 5 {
        println!("Incentive is N100,000");
        return
    }

    println!("Enter age:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Invalid value");
    
    if age < 28 {
       println!("Incentive is N1,300,000");
    }
    else if age >= 30 && age <= 39 {
        println!("Incentive is N1,480,000");
    }
    else if age >= 40 {
        println!("Incentive = N1,560,000");
    }
    else {
        println!("Invalid entry")
    }

    
    

}
