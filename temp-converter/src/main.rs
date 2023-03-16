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
