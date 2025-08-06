use std::io;

fn main() {
    println!(
        "This program is a Fibonacci sequence calculator. Input a number, and it will output that number in the sequence."
    );

    println!("Insert your number");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<i64>() {
        Ok(input) => {
            let fibonacci_number = calculate_fibo(input);
            println!("The fibonacci number for the position {input} is {fibonacci_number}");
        }
        Err(_) => println!("Invalid input. Please enter a valid number."),
    };
}

fn calculate_fibo(target: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..target {
        let temp = b;
        b = a + b;
        a = temp;
    }
    a
}
