fn print_greeting(greeting: &str) {
    println!("{}", greeting);
}

fn main() {
    let hello = String::new();
    print_greeting(&hello); // Passamos uma referência &str como argumento.
}
