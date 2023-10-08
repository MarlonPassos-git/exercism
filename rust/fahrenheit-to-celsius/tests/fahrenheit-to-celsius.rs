#[test]
fn test_fahrenheit_to_celsius() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    assert_eq!(fahrenheit_to_celsius(98.6), 37.0);
}

#[test]
fn test_celsius_to_fahrenheit() {
    assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    assert_eq!(celsius_to_fahrenheit(37.0), 98.6);
}
