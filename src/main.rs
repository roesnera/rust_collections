fn main() {
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
    println!("{item_from_tres}");

    // to iterate through a loop, you should use the for loop structure

    // note that we borrow here, rather than consume
    for i in &vector_tres {
        println!("{i}");
    };
}
