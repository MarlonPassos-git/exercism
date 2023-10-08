pub fn fahrenheit_to_celsius_v1(fahrenheit: f64) -> f64 {
    let result = (fahrenheit - 32.0) / 1.8;
    return result.round();
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // let result = (fahrenheit - 32.0) / 1.8;
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
