const ENGLISH_VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

fn main() {
    println!("{:?}", to_pig_latin("First"));
    println!("{:?}", to_pig_latin("Apple"));
    println!("{:?}", to_pig_latin(""));
    println!("{:?}", to_pig_latin("a"));
    println!("{:?}", to_pig_latin("f"));
    println!("{:?}", to_pig_latin("fe"));
    println!("{:?}", to_pig_latin("ff"));
    println!("{:?}", to_pig_latin("aa"));
    println!("{:?}", to_pig_latin("yellow"));
}

/// Converts word to a pig latin word. If a word starts with a vowel, it
/// adds `-hay` suffix else it removes the first character and adds
/// `-{first_character}ay` suffix.
///
/// Note: only works with single alphabetic words.
///
/// # Examples
///
/// ```
/// assert!(to_pig_latin("first") == "irst-fay");
/// assert!(to_pig_latin("apple") == "apple-hay");
/// ```
fn to_pig_latin(str: &str) -> String {
    let mut pig_latin_str = String::from(str);

    if str.len() < 2 {
        return pig_latin_str;
    }

    let first_char = str.chars().next().unwrap().to_lowercase().to_string();
    let is_first_char_vowel = ENGLISH_VOWELS.contains(&&first_char[..]);

    if is_first_char_vowel {
        pig_latin_str.push_str("-hay");
    } else {
        pig_latin_str.remove(0);
        pig_latin_str.push_str(&(String::from("-") + &first_char + "ay"));
    }

    pig_latin_str
}
