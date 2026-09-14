fn main() {
    let original = String::from("Hello Rust");

    let borrowed = &original;

    println!("Original: {}", original);
    println!("Borrowed: {}", borrowed);
}