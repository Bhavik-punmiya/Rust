fn main() {
    let sum: i32 = my_function(9, 45);

    let condition : bool = true;
    let number : i32 = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    println!("The sum of x and y is: {}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("{}", x);
    println!("The value of y is: {}", y);

    if x > y {
        println!("{} is greater than {} performing x + y", x, y);
        return x + y;
    } else if x < y {
        println!("{} is smaller than {} performing x - y", x, y);
        return x - y;
    } else {
        println!("{} is equal than {} perfroming x * y", x, y);
        return x * y;
    }
}
