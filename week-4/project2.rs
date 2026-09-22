use std::io;

fn main()  {
println!("Enter experience (1 for experiencd, 0 for not experienced)");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("failed to read input");
    let experienced:i32 = input1.trim().parse().expect("failed to input");

    println!("Enter age");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let age:i32 = input2.trim().parse().expect("failed to input");

    if experienced == 1 {
    if age >= 40 {
      println!("Annual incentive = N1,560,000");
    } else if age >= 30 && age <= 39 {
    println!("Annual incentive = N1,480,000");
    } else if age < 28 {
    println!("Annual incentive = N1,300,000");
    } else {
    println!("No matching incentive for this age");
    }
    }else {
    println!("Annual incentive = 100,000");
    }
}