use std::io;
 
fn main() {
    println!("Enter your weight (kg): ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let user_weight: f32 = user_input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(user_weight);
    println!("Weight on Mars is {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.711
}
