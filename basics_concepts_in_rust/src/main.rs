use std::time::Instant;

fn main() {
    // Record the start time
    let start_time = Instant::now();
    let mut x = 1;
    // Print "Hello World" 1000 times
    for _ in 0..100000 {
        x = x + 1;
        println!("Hello World {}", x);
    }
    let mut number = 100000;
    while number != 0 {
        println!("Hello World {}", number);
        number = number - 1;
    }
    // Record the end time
    let elapsed_time = start_time.elapsed();

    let a  = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value of element is: {}", element);
    }   

    // Print the elapsed time
    println!("\nTime taken to print 'Hello World' 200000 times: {:.4?} seconds", elapsed_time);
}
