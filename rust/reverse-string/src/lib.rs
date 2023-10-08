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
// has a little problem with this resolution, because with the word "b🇺🇸m" will turn "b🇸🇺m"
pub fn _reverse_v2(input: &str) -> String {
    input.chars().rev().collect()
}

// v3
// with this version is possible i handle with type of word "m🇺🇸b" => "b🇺🇸m"
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
