#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * struct Rectange{
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
        }   self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}


fn main(){
    let rec = Rectangle{
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle{
        width: 29,
        height: 30,
    };
    let rec3 = Rectangle{
        width: 56,
        height: 50,
    };

    println!("rect : {:#?}", rec);

    println!(
        "The area of the rectangle is {} square pixels.",
        rec.area()
    );


    println!("Can rec hold rec2? {}", rec.can_hold(rec2));
    println!("Can rec hold rec3? {}", rec.can_hold(rec3));


    let sq = Rectangle::square(3);
    println!("sq : {:#?}", sq);
}

