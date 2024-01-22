#[derive(Debug)]
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

fn ip_address(){
    let home = IpAddrKind::V4(192,168,0,20);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn main(){
    ip_address();
}


fn enum_value(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

