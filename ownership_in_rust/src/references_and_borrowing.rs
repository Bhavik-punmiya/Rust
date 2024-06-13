fn main(){
    let mut s1 = String::from("Hello");
    let len: usize = calculate_length(&mut s1);
    println!("The length of '{}' is: {}", s1, len);

    change(&mut s1);
    println!("The value of s1 is: {}", s1);

    multiple_mutable_references();
}

// Borrowing the value or references by & Value 
// References are immutable and cannot be changed
/*
    fn change(s: &String) {
        s.push_str(", World!");
    }

    will throw an error as references are immutable
*/



fn calculate_length(s: &String) -> usize {
    let length: usize= s.len();
    length
}

fn change(s: &mut String) {
    s.push_str(", World!");
}

// References are immutable by default but we can make them mutable by using &mut
// We can have only one mutable reference to a particular piece of data in a particular scope
// for example: 
/*
    let mut s = String::from("Hello");
    let r1 = &mut s;
    let r2 = &mut s;

    will throw an error as we have two mutable references to s
    We can have only one mutable reference to a particular piece of data in a particular scope
    This prevents the race Condition

    */

    // hey but note you can definetly do this 

    fn multiple_mutable_references() {
        let s = String::from("Hello");

        let r1 = &s;
        let r2 = &s;

        // Note a mutalbe reference if assigned to immutable reference it cannot be then reassigned to mutalbe reference

        /*
            let mut s = String::from("Hello"); mutable string

            let r1 = &s; immutalbe reference

            let mut r2 = &s; mutable reference

            This is not allowed as we have refered to first immutable reference and assigning again to immutable reference

        //  */
        // let mut s = String::from("Hello");

        // let r1 = &s;
        // let r2 = &mut s;
        // // over here r1 got dropped 

        println!("r1: {}, r2: {}", r1, r2);
    }



