fn main() {
    let c: f32 = fahrenheit_to_celsius(30.0);
    println!("Celsius: {c}");

    let f: f32 = celsius_to_fahrenheit(30.0);
    println!("Fahrenheit: {f}");
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    return celsius * 1.8 + 32.0;
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) / 1.8;
}
