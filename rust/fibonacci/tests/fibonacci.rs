use fibonacci::*;

#[test]
fn test_fibonacci_0() {
    assert_eq!(fibonacci(0), 0);
}

#[test]
fn test_fibonacci_1() {
    assert_eq!(fibonacci(1), 1);
}

#[test]
fn test_fibonacci_2() {
    assert_eq!(fibonacci(2), 1);
}

#[test]
fn test_fibonacci_3() {
    assert_eq!(fibonacci(3), 2);
}

#[test]
fn test_fibonacci_4() {
    assert_eq!(fibonacci(4), 3);
}

#[test]
fn test_fibonacci_5() {
    assert_eq!(fibonacci(5), 5);
}
