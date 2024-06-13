fn main() {
    let s: String = String::from("Hello, World!");  
    takes_ownership(s);
    // println!("{}", s); // This will throw an error
    // Here we have created "Hello, World!" on the heap and passed it to the function
    // The function takes ownership of the data and prints it
    // after the functions Ends that is Owner is removed , so Data is also removed 
    // So, if we try to print the data after the function ends, it will throw an error
    let x  = 5;
    makes_copy(x);
    println!("{}", x);
    // Here we have created 5 on the stack and passed it to the function
    // The function makes a copy of the data and prints it for int, bool, char and float 

    let mut str1 = String::from("Let's get Rusty");
    str1 = takes_and_gives_ownership(str1);
    println!("{}", str1);


    let new_owner = gives_ownership();
    println!("{}", new_owner);
    
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello, World!");
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}