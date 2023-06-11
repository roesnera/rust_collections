use crate::vectors::vectors;
use crate::strings::strings;
use crate::hash_maps::hash_maps;
pub mod strings;
pub mod vectors;
pub mod hash_maps;

fn main() {
    println!("Vector section begins below");
    println!("----------------------------------------------------------");
    // vectors();
    println!("String section begins below");
    println!("----------------------------------------------------------");
    strings();
    println!("Hash map section begins below");
    println!("----------------------------------------------------------");
    hash_maps();
}
