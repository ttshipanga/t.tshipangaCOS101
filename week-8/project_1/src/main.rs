////The APS Level Checker
use std::io;

fn main() {
    
    let categories = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];

    let mut role = String::new();
    println!("Choose Public Servant Category (0:Office Admin, 1:Academic, 2:Lawyer, 3:Teacher): ");
    io::stdin().read_line(&mut role).expect("WRONG!!");
    let role_index: usize = role.trim().parse().expect("WRONG!!");

    let mut years = String::new();
    println!("Enter years of experience:");
    io::stdin().read_line(&mut years).expect("WRONG!!");
    let years: u32 = years.trim().parse().expect("NO!");

    let chosen_role =categories[role_index];

    let level = match chosen_role {
        "Office Admin" => {
            if years < 2 {
                "APS Level 1"
            } else if years < 5 {
                "APS Level 2"
            } else if years < 10 {
                "APS Level 3"
            } else {
                "APS Level 4"
            }
        }
        "Academic" => {
            if years < 3 {
                "APS Level 1"
            } else if years < 6 {
                "APS Level 2"
            } else if years < 12 {
                "APS Level 3"
            } else {
                "APS Level 4"
            }
        }
        "Lawyer" => {
            if years < 4 {
                "APS Level 1"
            } else if years < 8 {
                "APS Level 2"
            } else if years < 15 {
                "APS Level 3"
            } else {
                "APS Level 4"
            }
        }
        "Teacher" => {
            if years < 5 {
                "APS Level 1"
            } else if years < 10 {
                "APS Level 2"
            } else if years < 20 {
                "APS Level 3"
            } else {
                "APS Level 4"
            }
        }
        _ => "Unknown Role",
    };
    println!("\nCategories: {}", chosen_role);
    println!("Years of Experience: {}", years);
    println!("APS Level: {}", level);
}


    