use fahrenheit_to_celsius::fahrenheit_to_celsius;

#[test]
fn test_fahrenheit_to_celsius() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    assert_eq!(fahrenheit_to_celsius(98.6), 37.0);
}
