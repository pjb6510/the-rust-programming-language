use std::collections::HashMap;

pub fn main() {
    let num_list = vec![1, 4, 2, 5, 6, 1, 9, 9, 9];

    let average = get_average(&num_list);
    let median = get_median(&num_list);
    let mode = get_mode(&num_list);

    println!("average {}", average);
    println!("median {}", median);
    println!("mode {}", mode);
}

fn get_average(num_list: &Vec<i32>) -> f32 {
    let mut sum: f32 = 0.0;
    for num in num_list {
        sum += *num as f32;
    }

    return sum / num_list.len() as f32;
}

fn get_median(num_list: &Vec<i32>) -> i32 {
    let mut sorted_num_list = num_list.clone();
    sorted_num_list.sort();

    let middle_index = sorted_num_list.len() / 2 as usize;

    // panic?
    return sorted_num_list[middle_index];
}

fn get_mode(num_list: &Vec<i32>) -> i32 {
    let mut appearance_count: HashMap<i32, i32> = HashMap::new();

    for num in num_list {
        let appearence = appearance_count.entry(*num).or_insert(0);

        *appearence += 1;
    }

    let mut max_num: i32 = i32::MIN;
    let mut max_appearance: i32 = i32::MIN;

    for (num, appearance) in appearance_count {
        if max_appearance <= appearance {
            max_num = num;
            max_appearance = appearance;
        }
    }

    return max_num;
}
