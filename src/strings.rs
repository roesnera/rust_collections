pub fn strings() {
    // Rust has one type of string in the core language
    // that is the str string slice
    // the String type is part of the standard library
    // Strings are growable, mutable, owned, UTF-8 encoded byte vectors with some added guarantees,
    // restrictions, and capabilities
    let mut string = String::new();
    // any type that implements the display trait can be made into a string using the to_string
    // method
    let data = "initial contents";
    let s = data.to_string();

    // this scope declared string will not be referencable outside of the block
    {
        let string2 = String::new();
    }

    // this results in an error
    // string2 = "hiya";
    // I would expect the below to no longer work
    println!("{data}");
    // It does. This means that the to_string() method performs a borrow and does not consume the
    // data that it is called on.
    // you can also call the .to_string() methoed on a literal directly
    string = "something".to_string();

}
