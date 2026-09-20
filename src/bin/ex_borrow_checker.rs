fn main() {
    let variable_1 = NewType{ value: String::from("Hello World!") };
    // creating an object and storing it in variable_1
    
    print_stuff(variable_1);
    // passing the object as a parameter to a function. this transfers the
    // ownership of the object from variable_1 to the function as
    // one of the parameters, which are internally represented by variables
    
//  print_stuff(variable_1);
    // Won't compile because the object stored in variable_1 is
    // no longer owned by variable_1, and as such cant be used anymore
    
    // because only 1 variable can hold and work with an object at a time, it means
    // that the program doesn't have to worry about things like objects being deleted
    // before their done being used or modified while being read. additionally, because
    // variables are automatically cleaned up when they aren't used anymore or fall
    // out of scope, it means that there's no risk of accidentally forgetting to
    // clean up an object after it's done being used. this acts as a sort of
    // inline garbage collector.
    
    //--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//--//
    
    // there are many times however where you just need temporary access to an object,
    // which is why rust created the borrowing system.
    
    let variable_2 = NewType{ value: String::from("Foo Bar!") };
    // creating another object and storing it in variable_2
    
    let borrowed = &variable_2;
    // by using the '&' symbol, the object in variable_2 is borrowed, letting it
    // be used elsewhere in the program even though It's still owned by variable_2.
    
    print_from_borrow(borrowed);
    // because borrowed values are a separate type, functions have to specify whether
    // they take a normal or borrowed value.
    
    let borrowed_2 = &variable_2;
    print_from_borrow(borrowed_2);
    // because borrows are read-only by default, multiple can be created and used
    // at the same time.
    
    print_from_borrow(borrowed);
    // similarly, borrows can be reused, as they can only be used to read data.
    // this means that they can be used kind of like a standard object reference
    // in other languages.
    
    let mut variable_3 = NewType{ value: String::from("Not Mutated Yet!") };
    // this changes with mutable references
    
    let borrowed_mutable = &mut variable_3;
    // mutable borrows are made similarly to normal borrows, just with the extra 'mut'
    
//  let borrowed_3 = &variable_3;
//  let borrowed_mutable_2 = &mut variable_3;
    // while a mutable borrow is active, no other borrows can be created. similarly,
    // mutable borrows can only be created if there are no other active borrows
    
    mutate_from_borrow(borrowed_mutable);
    // mutable borrows also have their own type separate from normal borrows.
    
    print_from_borrow(&variable_2)
    // after the last use of a borrow, it is automatically cleaned up, letting
    // it be borrowed by other things again.
}
struct NewType{
    value:String
}
fn print_stuff(object:NewType){
    println!("{}", object.value)
}
fn print_from_borrow(object:&NewType){
    println!("{}", object.value)
}
fn mutate_from_borrow(object:&mut NewType){
    object.value=String::from("Mutated!")
}
