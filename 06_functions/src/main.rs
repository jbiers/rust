fn main() {
    print_labeled_measurement(30, 'm');

    let x = five();
    println!("{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// note the differenct statement vs expression
fn five() -> i32 {
    5
}