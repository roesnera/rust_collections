use std::collections::HashMap;

pub fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // this doesn't work due to hashmaps not being formatable by the default formatter
    // println!("{scores}");
    
    // we can retreive values using the get method on the hashmap and providing a string slice
    let key = String::from("Blue");
    let value = match scores.get(&key).copied() {
        Some(val) => val,
        None => 0000
    };

    println!("{value}");

    // you can also iterate over a hash map using a for loop
    //for (key, value) in &scores {
    //    println!("{key}: {value}");
    //};


    // values with the copy trait will not be owned by the hashmap by default
    // values that do not implement the copy trait (e.g. strings) their ownership is transferred
    // for example, I will insert at the key specified by a string variable some arbitrary value
    // then I will no longer be able to reference that variable
    let key = String::from("Red");
    scores.insert(key, 25);


    //for (key, value) in &scores {
    //    println!("{key}: {value}");
    //};

    // I expect the statement below to throw an error
    // println!("{key}");
    
    // there are multiple ways to update a hash map
    //  - replace a value
    //  - place a value if one does not already exist
    //  - manipulate a value based upon an old value
    // I have already shown the insert method, which inserts a new value

    // the entry method allows you to perform an insert only if a value doesn't exist using the
    // .or_insert method

    scores.entry(String::from("Yellow")).or_insert(666);
    scores.entry(String::from("Mauve")).or_insert(666);

    
    for (key, value) in &scores {
        println!("{key}: {value}");
    };

    // the above loop will show that the value at Yellow is unchanged, 
    // but and entry at key "Mauve" has been created with a value of 666


    // you can also insert a value based on an old value at that location

    let mut new_map = HashMap::new();

    for word in "hello world wonderful world".split_whitespace() {
        let count = new_map.entry(word).or_insert(0);
        // here we have to dereference count because it will fall out of scope at the end of the
        // loop iteration
        //
        *count += 1;
    }
    println!("{:?}",new_map);
}
