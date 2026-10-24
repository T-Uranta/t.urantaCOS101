
use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO101110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanco Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut file = File::create("students.txt").unwrap();

    for student in &students {
        println!(
            "{} | {} | {} | {}",
            student.0, student.1, student.2, student.3
        );

        writeln!(
            file,
            "{} | {} | {} | {}",
            student.0, student.1, student.2, student.3
        ).unwrap();
    }

    println!("Student details saved.");
}
