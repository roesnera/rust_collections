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

pub fn convert_strings(string: String) -> String {
    println!("{string}");

    let words = string.split(" ");

    let words_collection: Vec<&str> = words.collect();
    let mut words_as_strings: Vec<String> = Vec::new();

    for word in words_collection {
        println!("{word}");
        let mut word_as_string: String;
        if word.starts_with("a") || word.starts_with("e") || word.starts_with("i") || word.starts_with("o") || word.starts_with("u") || word.starts_with("A") || word.starts_with("E") || word.starts_with("I") || word.starts_with("O") || word.starts_with("U"){ 
            word_as_string= word[..].to_string();
        } else {
            word_as_string = word[1..].to_string();
        }
        word_as_string.push('-');
        word_as_string.push_str(&word[0..1]);
        word_as_string.push_str("ay");
        words_as_strings.push(word_as_string);
    }

    println!("{:?}", words_as_strings);

    let mut final_pig_latin_string: String = String::new();

    for word in words_as_strings {
        final_pig_latin_string.push_str(&word[..]);
        final_pig_latin_string.push(' ');
    }

    return final_pig_latin_string;
}
