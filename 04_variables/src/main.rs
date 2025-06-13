// const JULIA: &str = "Julia";
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = 7;
    println!("The value of x is {x}");

    // The difference between an immutable variable and a constant
    // is that a constant can't be shadowed in the same scope
    // Also, constants can only be set to a constant expression
    const Y: i32 = 10; 
    println!("The value of Y is {Y}");
}
