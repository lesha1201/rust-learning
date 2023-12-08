use std::collections::HashMap;

fn main() {
    let first_list = [1, 2, 3, 4, 5];
    let second_list = [2, 4, 5, 1, 3];
    let third_list = [1, 2];
    let fourth_list = [1, 1, 1, 3];
    let fifth_list = [3, 1, 3, 2];

    println!("First list: {:?}", &first_list);
    println!("First list median: {}", median(&first_list));
    println!("First list mode: {:?}", mode(&first_list));
    println!();

    println!("Second list: {:?}", &second_list);
    println!("Second list median: {}", median(&second_list));
    println!("Second list mode: {:?}", mode(&second_list));
    println!();

    println!("Third list: {:?}", &third_list);
    println!("Third list: {}", median(&third_list));
    println!("Third list mode: {:?}", mode(&third_list));
    println!();

    println!("Fourth list: {:?}", &fourth_list);
    println!("Fourth list median: {}", median(&fourth_list));
    println!("Fourth list mode: {:?}", mode(&fourth_list));
    println!();

    println!("Fifth list: {:?}", &fifth_list);
    println!("Fifth list median: {}", median(&fifth_list));
    println!("Fifth list mode: {:?}", mode(&fifth_list));
}

/// Returns median of the provided list.
///
/// # Examples
///
/// ```
/// let odd_list = [1, 2, 3];
/// let even_list = [1, 2];
///
/// assert!(median(&odd_list) == 2.0);
/// assert!(median(&even_list) == 1.5);
/// ```
fn median(list: &[i32]) -> f64 {
    let mut list_vec = list.to_vec();

    list_vec.sort();

    let list_length = list_vec.len();
    let middle_index = list_vec.len() / 2;

    assert!(list_length > 0);

    if list_length % 2 == 0 {
        return (list_vec[middle_index - 1] + list_vec[middle_index]) as f64 / 2.0;
    } else {
        return list_vec[middle_index] as f64;
    }
}

/// Return mode (the value that occurs the most) of the provided list. If all
/// values occur equally, it returns `None`.
///
/// # Example
///
/// ```
/// assert!(mode(&[1, 1, 3]) == Some(1));
/// assert!(mode(&[1, 2, 3]) == None);
/// ```
fn mode(list: &[i32]) -> Option<i32> {
    let mut list_item_count_map = HashMap::new();

    let mut highest_item: Option<i32> = None;
    let mut highest_item_count: i32 = 0;

    for item in list {
        let count = list_item_count_map.entry(item).or_insert(0);
        *count += 1;

        if *count > highest_item_count {
            highest_item_count = *count;
            highest_item = Some(*item);
        } else if *count == highest_item_count {
            highest_item = None;
        }
    }

    highest_item
}
