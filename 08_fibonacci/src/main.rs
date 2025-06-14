use std::io;

fn main() {
    let mut n: String = String::new();

    println!("Input a number for the Fibonacci sequence:");

    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read number.");

    let n: u32 = n
    .trim()
    .parse()
    .expect("Please provide an integer.");

    let result: u32 = fibonacci(n);
    println!("The {n}th Fibonacci number is {result}")
}

fn fibonacci(n: u32) -> u32 {
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;

    if n == 1 {
        return n1;
    }

    if n == 2 {
        return n2;
    }

    let mut result: u32 = 0;
    for _i in 1..n {
        result = n1 + n2;
        n1 = n2;
        n2 = result;
    }

    return result;
}
