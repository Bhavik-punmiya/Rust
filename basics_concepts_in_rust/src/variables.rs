
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
     x = 6;
 
     println!("The value of x is {}", x);
 
     // constant key word declaration
     const SUBSCRIBER_COUNT : u32 = 1000000;
     println!("The number of subscribers is: {}", SUBSCRIBER_COUNT);
 
     // difference between const and let
     // const is always immutable and cannot use mut keyword
     // you must always assign a type to a const
     
 
     // shadowing
     let y = 5;
     println!("The value of y is: {}", y);
     let y = "hello";
     println!("The value of y is: {}", y);
 
     // Scaler and Compound DataTypes 
 
     /*
         Integers - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, isize, usize
         Floating Numbers
         Booleans
         Characters
     */
 
     let a : i32 = 98_222; // Decimal
     let b : i32 = 0xff; // Hex
     let c : i32 = 0o77; // Octal
     let d : i32 = 0b1111_0000; // Binary
     let e = b'A'; // Byte (u8 only)
 
 
     let f: u8 = 255;
     
     let g: f32 = 2.0; // f64
 
     let h: bool = true;
 
     let i: char = 'z';
 
     // Compound DataTypes
 
     let tup: (&str, f64) = ("Let's Learn Rust", 6.4);
     let (channel, sub_count ) = tup;
     let sub_count = tup.1;
 
 
 
 
 
 }
 