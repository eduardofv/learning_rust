use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn main() {
    let mut hasher = DefaultHasher::new();

    hasher.write(b"Hola");
    println!("Hash is {:x}", hasher.finish());
}
