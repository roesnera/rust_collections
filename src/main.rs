use crate::vectors::vectors;
use crate::strings::strings;
use crate::hash_maps::hash_maps;
use crate::practice_problems::*;
pub mod strings;
pub mod vectors;
pub mod hash_maps;
pub mod practice_problems;

fn main() {
    println!("Vector section begins below");
    println!("----------------------------------------------------------");
    // vectors();
    println!("String section begins below");
    println!("----------------------------------------------------------");
    // strings();
    println!("Hash map section begins below");
    println!("----------------------------------------------------------");
    // hash_maps();
    println!("Practice problem section begins below");
    println!("----------------------------------------------------------");
    // let vector = median(&[1,2,3,2,1]);
    // println!("{:?}", vector);

    let pig_latin = convert_strings(String::from("Hello world"));
    println!("{pig_latin}");
}
