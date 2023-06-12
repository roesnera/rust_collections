use std::collections::HashMap;
pub fn median(list: &[i32]) -> Vec<i32> {
    println!("List called!");
    let mut vector_to_return = vec![0, 0];
    let mut frequency_counter = HashMap::new();
    let mut sum = 0;
    let mut len = 0;
    for num in list {
        let count = frequency_counter.entry(num).or_insert(0);
        *count += 1;
        sum += num;
        len += 1;
    }
    println!("sum is: {sum}, and length is: {len}");
    let mut max_count = 0;
    let mut prev_max_entry = 0;
    for (key, value) in &frequency_counter {
        if value > &max_count {
            max_count = *value;
            prev_max_entry = **key;
        };
    }
    println!("max count is: {max_count}, and the number that exhibits that count is: {prev_max_entry}");
    println!("{:?}",frequency_counter);
    vector_to_return[1] = prev_max_entry;
    vector_to_return[0] = sum / len;
    return vector_to_return;
}
