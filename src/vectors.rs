pub fn vectors() {

    // vectors are like an array
    // one type of data is stored in a location on the heap
    
    let v: Vec<i32> = Vec::new();
    // the type annotation here is not super common in practice, more for demonstration purposes
    // rust provides the vec! macro to create a new vector with the given initial values
    
    let other_v = vec![1,2,3];

    println!("{:?}", other_v);
    println!("{:?}", v);

    // just like with other values, vectors must be declared as mutable in order to have their
    // values manipluated after instantiation
    // the below will not work, since v is not declared as mutable
    // v.push(1);
    
    // this is allowed
    let mut vector_tres = vec![0];
    vector_tres.push(1);

    println!("{:?}", vector_tres);

    // you can reference an item in a vector using index referencing or using the get method
    
    let third_item: &i32 = &other_v[2];
    println!("{}", third_item);

    // the get method will return an option so you have to deal with that
    // here we just unwrap, but we should use match
    let second_item: &i32 = &other_v.get(1).unwrap();
    println!("{}", second_item);

    let first_item: &i32 = match &other_v.get(0) {
        Some(i) => i,
        None => &-1
    };
    println!("{}", first_item);

    // mutable vs immutable borrow rules still apply
    // if you try to use an immutable reference after make a mutable reference such as .push()
    // mutable references occur even when we don't explicitly pass the reference to a method or fn
    // if a method refers to Self in a mutable way, that counts as a mutable reference
    
    // e.g. below I immutably borrow data from vector_tres and save it to a variable
    let item_from_tres = &vector_tres[1];

    // I am allowed to print it and use it in immutable ways as I need
    println!("{item_from_tres}");

    // but once I mutate the vector the data is borrowed from, in this case using .push()
    vector_tres.push(2);

    // I can no longer reference item_from_tres because vector_tres has been mutated
    // println!("{item_from_tres}");

    println!("--------------------------------------------------------");
    // to iterate through a loop, you should use the for loop structure

    // note that we borrow here, rather than consume
    for i in &vector_tres {
        println!("{i}");
    };

    // we are not allowed to insert or remove items from a vector in a for loop
    // but we are perfectly well allowed to mutate the original vector's values
    
    for i in &mut vector_tres {
        *i += 10;
        println!("{i}");
    };

    println!("--------------------------------------------------------");
    // vectors can only store one type of data, but we can leverage enums to skirt this limitation

    #[derive(Debug)]
    enum VariableData {
        Int(i32),
        Text(String),
    } 

    let some_data = vec![
        VariableData::Int(16),
        VariableData::Text(String::from("hello, there!")),
    ];

    for i in &some_data {
        println!("{:#?}",i);
    }
}
