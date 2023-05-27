fn main() {
    let number_list = vec![32, 43, 234, 42, 3];

    let result = largest(&number_list);

    println!("The largest number from {:?} is {}", number_list, result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);

    println!("The largest char from {:?} is {}", char_list, result);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
