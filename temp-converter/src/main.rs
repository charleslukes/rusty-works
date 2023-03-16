use std::io;

fn main() {
    println!("Welcome To Temperature Converted");
    println!("State what temperature, you want to convert into. F or C ?");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Invalid input");
    let temp = temp
        .trim()
        .to_lowercase()
        .chars()
        .next()
        .expect("Not valid");
    println!("Enter temperature value");
    let mut temp_value = String::new();
    io::stdin()
        .read_line(&mut temp_value)
        .expect("Invalid input");
    // reused `temp_value` now I see the benefit of shadowing
    let temp_value: i32 = temp_value.trim().parse().expect("Not a valid number");
    let converted_temp = temp_converter(temp, temp_value as f32);
    println!("The conversion is {converted_temp}")
}

fn temp_converter(temp_to_convert_into: char, value: f32) -> String {
    let result: f32 = if temp_to_convert_into == 'f' {
        value * 9.0 / 5.0 + 32.0
    } else if temp_to_convert_into == 'c' {
        ((value - 32.0) * 5.0) / 9.0
    } else {
        0.0
    };

    result.to_string() + "°" + &String::from(temp_to_convert_into)
}
