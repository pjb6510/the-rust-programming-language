fn main() {
    let num_list = vec![1, 4, 2, 5, 4, 23, 5, 234];
    let char_list = vec!['a', 'b', 'c', 'd', 'e'];

    let largest_number = largest(&num_list);
    let largest_char = largest(&char_list);

    println!("largest_number {}", largest_number);
    println!("largest_char {}", largest_char);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut result = list[0];

    for &item in list.iter() {
        if item > result {
            result = item;
        }
    }

    return result;
}
