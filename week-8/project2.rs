use std::io;

fn main() {
    let mut people: Vec<(String, u32)> = Vec::new();

    println!("How many people are being interviewed?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u32 = input.trim().parse().unwrap();

    for i in 0..number {
        let mut name = String::new();
        let mut years = String::new();

        println!("Enter person's name:");
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter years of experience:");
        io::stdin().read_line(&mut years).unwrap();

        let years: u32 = years.trim().parse().unwrap();

        people.push((name.trim().to_string(), years));
    }

    let mut highest = &people[0];

    for person in &people {
        if person.1 > highest.1 {
            highest = person;
        }
    }

    println!("Most experienced person: {}", highest.0);
    println!("Years of experience: {}", highest.1);
}
