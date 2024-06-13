enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddrKind {
    kind: IpAddrKind,
    address: String,
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}



impl Message{
    fn some_function (){
        println!("Let's Get Rusty");
    }
}

fn main(){
    let localhost : IpAddrKind = IpAddrKind {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1");
    };

    enum Option<T> {
        Some(T), 
        None,
    }



}

fn route(ip_kind: IpAddrKind){

}