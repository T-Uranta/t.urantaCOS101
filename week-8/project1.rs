use std::io;

fn main() {
    let categories = vec![
        "office admin", "academic", "lawyer", "teacher"
    ];

    let jobs = vec![
        vec!["intern", "administrator", "senior administrator",
             "office manager", "director", "ceo"],

        vec!["-", "research assistant", "phd candidate",
             "post-doc researcher", "senior lecturer", "dean"],

        vec!["paralegal", "junior associate", "associate",
             "senior associate 1-2", "senior associate 3-4", "partner"],

        vec!["placement", "classroom teacher", "senior teacher",
             "leading teacher", "deputy principal", "principal"]
    ];

    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8",
                      "EL1 8-10", "EL2 10-13", "SES"];

    let mut category = String::new();
    let mut job = String::new();

    println!("Enter job category:");
    io::stdin().read_line(&mut category).unwrap();

    println!("Enter job title:");
    io::stdin().read_line(&mut job).unwrap();

    let category = category.trim().to_lowercase();
    let job = job.trim().to_lowercase();

    let mut found = false;

    for i in 0..categories.len() {
        if category == categories[i] {
            for j in 0..jobs[i].len() {
                if job == jobs[i][j] {
                    println!("APS Level: {}", levels[j]);
                    found = true;
                }
            }
        }
    }

    if !found {
        println!("Invalid category or job title.");
    }
}
