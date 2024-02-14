pub mod vectors;
pub mod enums;
pub mod strings;
pub mod hash_map;
pub mod exercise;


use vectors::vector;
use enums::enum_type_vector;
use strings::string_type;
use hash_map::hash_map;
use exercise::exercises;

fn main() {
    println!("Hello, world!");
    vector();
    enum_type_vector();
    string_type();
    hash_map();
    exercises();
}
