
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("breweries.txt").unwrap();

    writeln!(file, "NIGERIAN BREWERIES").unwrap();

    writeln!(file, "\nLAGER").unwrap();
    writeln!(file, "33 Export").unwrap();
    writeln!(file, "Desperados").unwrap();
    writeln!(file, "Goldberg").unwrap();
    writeln!(file, "Gulder").unwrap();
    writeln!(file, "Heineken").unwrap();
    writeln!(file, "Star").unwrap();

    writeln!(file, "\nSTOUT").unwrap();
    writeln!(file, "Legend").unwrap();
    writeln!(file, "Turbo King").unwrap();
    writeln!(file, "Williams").unwrap();

    writeln!(file, "\nNON-ALCOHOLIC").unwrap();
    writeln!(file, "Maltina").unwrap();
    writeln!(file, "Amstel Malta").unwrap();
    writeln!(file, "Malta Gold").unwrap();
    writeln!(file, "Fayrouz").unwrap();

    println!("File created successfully.");
}
