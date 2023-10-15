const NUMBERS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const MIDDLE_SENTENCES: [&str; 11] = [
    "Twelve drummers drumming,",
    "Eleven pipers piping,",
    "Ten lords a-leaping,",
    "Nine ladies dancing,",
    "Eight maids a-milking,",
    "Seven swans a-swimming,",
    "Six geese a-laying,",
    "Five golden rings,",
    "Four calling birds,",
    "Three French hens,",
    "Two turtle doves,",
];

pub fn the_twelve_days_of_chrisms() -> String {
    let mut text = String::new();

    for ordinal_number in NUMBERS {
        // aqui parece que estou usando o & pois estou convertendo String to &str
        text.push_str(&get_initial_part_of_letter(ordinal_number));
        text.push_str(&get_middle_part_of_letter(ordinal_number));
        text.push_str(&get_final_part_of_letter(ordinal_number));
    }

    return text;
}

fn find_index_of_word(word: &str) -> Option<usize> {
    for (index, &item) in NUMBERS.iter().enumerate() {
        if item == word {
            return Some(index);
        }
    }
    return None;
}

fn get_initial_part_of_letter(ordinal_number: &str) -> String {
    return format!("On the {ordinal_number} day of Christmas,\nmy true love sent to me\n");
}

fn get_middle_part_of_letter(ordinal_number: &str) -> String {
    let index = find_index_of_word(ordinal_number).expect("Nao achei");
    let mut text = String::new();

    for index2 in 0..index {
        let part = MIDDLE_SENTENCES[MIDDLE_SENTENCES.len() - 1 - index2];
        text = format!("{part}\n{text}");
    }

    return text;
}

fn get_final_part_of_letter(ordinal_number: &str) -> String {
    let prefix = if ordinal_number == "first" {
        "A"
    } else {
        "And a"
    };
    let posfix = if ordinal_number == "twelfth" {
        "!\n"
    } else {
        ".\n\n"
    };
    return format!("{prefix} partridge in a pear tree{posfix}");
}
