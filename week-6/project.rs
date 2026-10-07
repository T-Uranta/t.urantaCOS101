use std::io;

fn main() {
    println!("P - Poundo Yam - 3200");
    println!("F - Fried Rice - 3000");
    println!("A - Amala - 2500");
    println!("E - Eba - 2000");
    println!("W - White Rice - 2500");

    let mut food = String::new();
    println!("Enter food:");
    io::stdin().read_line(&mut food).unwrap();

    let mut quantity = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut quantity).unwrap();

    let quantity: i32 = quantity.trim().parse().unwrap();

    let price = if food.trim() == "P" {
        3200
    } else if food.trim() == "F" {
        3000
    } else if food.trim() == "A" {
        2500
    } else if food.trim() == "E" {
        2000
    } else {
        2500
    };

    let total = price * quantity;

    if total > 10000 {
        println!("Total = {}", total * 95 / 100);
    } else {
        println!("Total = {}", total);
    }
}