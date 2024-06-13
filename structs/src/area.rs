struct Rectange{
    width: u32,
    height: u32,
}


fn main(){
    let rec = Rectange{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec)
    );

    

}

fn area(rect: &Rectange) -> u32 {
    rect.width * rect.height
}   