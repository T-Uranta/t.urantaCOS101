
use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc.txt").unwrap();

    for i in 0..commissioners.len() {
        println!(
            "{} | {} | {}",
            commissioners[i], ministries[i], zones[i]
        );

        writeln!(
            file,
            "{} | {} | {}",
            commissioners[i], ministries[i], zones[i]
        ).unwrap();
    }

    println!("EFCC records saved.");
}
