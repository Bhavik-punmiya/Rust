fn main(){
    let x  = 'a';
    let y = x;

    // Now here we have done Copy 
    // In Rust Copy is allowed for Scalar DataTypes that are int, float, bool, char 
    // They're Copied and stored on the stack

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);


    let s1 = String::from("Hello");
    let s2 = s1;

    // Now here we have done Move
    // In Rust Move , s1 gets invalidated and s2 is the owner of the data
    // s1 is stored on the heap and s2 is the owner of the data
    // s1 is removed or invalidated 

    // println!("The value of s1 is: {}", s1); // This will throw an error

    println!("The value of s2 is: {}", s2);

}