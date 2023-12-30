//classic enum
enum IpAddresKind  {
    V4,
    V6,
}
//Enum carrying a value
enum IpAddrEnum {
    V4(String),
    V6(String),
}
//creation of a struct with enum
struct IpAddr {
    kind: IpAddresKind,
    address:String,
}
//enums can have multiple assigned vallues
enum IpAddresses {
    V4(u8,u8,u8,u8),
    V6(String),
}


//large enum
enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
//ad a function to an enum like this
impl Message {
    fn call(&self){
        //something
    }
}

fn main() {

    //using an enum in a struct to set the ipAddres version
    let local = IpAddr{
        kind: IpAddresKind::V4,
        address: String::from("127.0.0.1")
    };
    let raspbp = IpAddr{
        kind: IpAddresKind::V4,
        address:String::from("192.168.1.47")
    };

    //setting the value with the type witouth the use of a struct
    let loopback = IpAddrEnum::V6(String::from("::1"));
    let sql = IpAddrEnum::V4(String::from("192.168.1.46"));

    //using an enum with multiple values and 2 different types depending on the enum choice
    let loopback = IpAddresses::V4(127,0,0,1);
    let loopback = IpAddresses::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();



}
