use std::io;


fn main() {
    println!("Welcome to Tships' Restaurant!!!");
    println!("Here's the menu:");
    println!("P - Poundo Yam / Edinkaiko Soup - N3,200");
    println!("F - Fried Rice & Chicken - N3,000");
    println!("A - Amala & Ewedu Soup - N2,500");
    println!("E - Eba & Egusi Soup - N2,000");
    println!("W - White Rice & Stew - N2,500");

    //Prices
    println!("Make your choice NOW!:");
    println!("Enter food code(P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read input");
    let food_code = food_code.trim().to_uppercase();

    let price_per_item = match food_code.as_str() {
        "P" => 3200.00,
        "F" => 3000.00,
        "A" => 2500.00,
        "E" => 2000.00,
        "W" => 2500.00,
        _ => {
            println!("Invalid selection!");
            return;
        }
    
    };
    //Quantityy
    print!("Enter quantity: ");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read input");

    let quantity: f64 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity number!");
            return;
        }
    };

    let mut total_charge = price_per_item * quantity;
    println!("\nSubtotal: N{:.2}", total_charge);

    if total_charge > 10000.00 {
        let discount = total_charge * 0.05;
        total_charge -= discount;
        println!("Discount Applied (5%): -N{:.2}", discount);
    
    }else{
        println!("Discount Applied: None");
    }

    println!("Total Bill: N{:.2}", total_charge);
    
    
}

