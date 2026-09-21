fn main() {
    // sometimes programs run into errors, and it's important to handle them correctly so
    // the program can continue to run as expected.
    
    let mut result: Result<i32,()>;
    result = Err(());
    result = Ok(115);
    // the default form of error handling is with a Result. it's an enum with 2 cases:
    // Ok and Err. it's used in functions where there might be a small error. if there
    // is no error, it returns an Ok storing the output, and if there is, then it returns
    // an Err. Err's can also have values, but typically are just units, denoted by '()'
    
    match result {
        Ok(value) => {println!("Ok: {}",value)}
        Err(_) => {println!("Error!")}
    }
    // just like other enums, they can be used in match statements to do different things
    // weather there's an error or not.
    
    let unwrapped_result = result.unwrap();
    // results can also be unwrapped, returning the value if it is Ok or causing a panic
    // if it's an error. there are also many similar methods such as unwrap_or_else() and
    // unwrap_or_default() for more robust unwrapping.
    
    print!("{}", unwrapped_result);
    
    let mut option:Option<i32>;
    // there are many times when a value might be null, but in rust, null is not normally
    // allowed. to get around this, an Option<> is used.
    
    option = None;
    option = Some(4096);
    // options are also enums with 2 cases: Some() and None. some is used when the option
    // is NOT null and none is used when the option IS null.
    
    if option.is_some() {
        let value = option.unwrap();
        print!("Value: {}",value)
    }
    else{
        print!("Value is Null")
    }
    // Option.is_some() tests if the option has a non-null value. Option.unwrap() is used
    // to extract the value if it is not null if it IS null, then it will cause a panic.
    
    panic!("Why is it breaking :( ")
    // panics are a lot closer to a standard exception, repeatedly returning from the
    // current function until the thread or program closes and optionally showing the
    // stack trace. unlike in java, panics cannot be caught, so it's generally recommended
    // to use Result unless it's a very serious error.
}