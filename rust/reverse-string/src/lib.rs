// v1
pub fn _reverse_v1(input: &str) -> String {
    let mut reverse_string = String::from("");

    for c in input.chars().rev() {
        dbg!(c);
        reverse_string.push(c)
    }

    reverse_string
}

// v2
pub fn _reverse_v2(input: &str) -> String {
    input.chars().rev().collect()
}

// v3
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
