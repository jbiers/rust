use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // If element > len(a), the program will panic
    // This is an example of Rust's memory safery features
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
