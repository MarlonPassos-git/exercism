mod lib;

fn main() {
    let input = "m🇺🇸b";
    let input = lib::reverse(input);
    println!("result: {}", input);
}
