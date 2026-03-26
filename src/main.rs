// Convert strings to Pig Latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    println!("Select language:");
    println!("English - 1");
    println!("Русский - 2");

    // TODO Enum for langs. Обработка ввода и установление значения переменной языка
    //
    println!("Input one word: ");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Error in user input");

    let word = format!("{}-{}",safe_tail_slice(&user_input.trim(), 1), safe_slice(&user_input.trim(), 1));

    println!("{word}");
}


fn safe_slice(s: &str, char_count: usize) -> &str {
    match s.char_indices().nth(char_count) {
        Some((byte_idx, _)) => &s[..byte_idx],
        None => s, // Вернуть всю строку, если символов меньше
    }
}

fn safe_tail_slice(s: &str, first_char: usize) -> &str {
    match s.char_indices().nth(first_char) {
        Some((byte_idx, _)) => &s[byte_idx..],
        None => s, // Вернуть всю строку, если символов меньше
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}
