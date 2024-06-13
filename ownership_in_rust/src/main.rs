fn main(){
    let mut s = String::from("Hello, World!");
    let hello = &s[0..5];
    let _world = &s[7..12];
    let _s2 = "Hello Rust";
    let word: &str= first_word(&_s2);
    println!("The value of hello is: {}", hello);
}


fn first_word(s: &String)-> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}