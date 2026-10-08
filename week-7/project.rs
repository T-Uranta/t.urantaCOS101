use std::io;

fn read() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn trapezium() {
    println!("Enter height:");
    let h = read();
    println!("Enter base 1:");
    let b1 = read();
    println!("Enter base 2:");
    let b2 = read();

    println!("Area = {}", h / 2.0 * (b1 + b2));
}

fn rhombus() {
    println!("Enter diagonal 1:");
    let d1 = read();
    println!("Enter diagonal 2:");
    let d2 = read();

    println!("Area = {}", 0.5 * d1 * d2);
}

fn parallelogram() {
    println!("Enter base:");
    let b = read();
    println!("Enter height:");
    let h = read();

    println!("Area = {}", b * h);
}

fn cube() {
    println!("Enter side:");
    let s = read();

    println!("Surface area = {}", 6.0 * s * s);
}

fn cylinder() {
    println!("Enter radius:");
    let r = read();
    println!("Enter height:");
    let h = read();

    println!("Volume = {}", std::f64::consts::PI * r * r * h);
}

fn main() {
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");
    println!("Choose a shape:");

    let choice = read() as i32;

    match choice {
        1 => trapezium(),
        2 => rhombus(),
        3 => parallelogram(),
        4 => cube(),
        5 => cylinder(),
        _ => println!("Invalid choice"),
    }
}