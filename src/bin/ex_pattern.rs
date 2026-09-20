use rand;

fn main() {
    // pattern matching is important for many larger programs. think of it like
    // an if statement for data types other than boolean.
    
    let number = rand::random_range(1..5);
    
    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        _ => println!("Something else"),
    }
    // the '|' operator is used to separate options that both perform the same block of code.
    // the '_' value is used as a catch-all case for any input that doesn't
    // match anything else.
    
    
    enum Type {
        NoValue,
        IntValue(i32),
        StringValue(String),
    }
    // enums are a way to name certain options in your code to increase readability.
    // they can also store a value alongside it which can be used after checking if
    // the enum is that type.
    
    fn print_value(input: Type){
        match input {
            Type::NoValue => {println!("No value")}
            Type::IntValue(value) => {println!("Integer with value {}",value)}
            Type::StringValue(value) => {println!("String with value {}",value)}
        }
    }
    // just like before, it checks which type it has and does the corresponding action.
    // because enums usually have a small number of cases, its possible can check
    // all of them, meaning you don't need to use a catch-all case. the individual cases
    // can also work with any stored data the enum case contains.
    
    let test1 = Type::NoValue;
    let test2 = Type::IntValue(2048);
    let test3 = Type::StringValue(String::from("This is a string value!"));
    print_value(test1);
    print_value(test2);
    print_value(test3);
}