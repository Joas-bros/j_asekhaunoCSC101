
use std::io;

fn main() {
    
    let mut prices = [
        ("Poundo Yam/Edinkaiko Soup", 3200),
        ("Fried Rice & Chicken", 3000),
        ("Amala & Ewedu Soup", 2500),
        ("Eba & Egusi Soup", 2000),
        ("White Rice & Stew", 2500),
    ];

    
    println!("What type of food would you like to order?");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");

    
    let mut food_price = 0;
    for (f, p) in prices.iter() {
        if f == &food_type.trim() {
            food_price = *p;
            break;
        }
    }

    
    println!("How many {}s would you like to order?", food_type.trim());
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read input");
    let quantity: u64 = quantity_str.trim().parse().expect("Please enter a valid number");

    
    let total_charge:f64 = (food_price * quantity) as f64;

    
    let final_charge = if total_charge > 10000.0{
        total_charge * 0.95
    } else {
        total_charge
    };

    println!("The total charge for your order is: {}", final_charge);
}

