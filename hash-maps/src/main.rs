use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Red"), 25);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(25);

    println!("{:?}", scores);

    match scores.get_mut(&String::from("Blue")) {
        Some(blue_score) => *blue_score += 1,
        _ => (),
    }

    println!("{:?}", scores);
}
