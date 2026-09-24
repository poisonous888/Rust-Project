use rand;
use std::io;

fn main() {
    println!("Hello, world!");
    let random_test = rand::random::<i32>();
    println!("{:?}", random_test);
    let input = io::stdin();
    let mut guess = String::new();
    input.read_line(&mut guess).expect("TODO: panic message");
    println!("{1}{0}",guess,"Input: ");
    if 1>2 {
        
    }
}