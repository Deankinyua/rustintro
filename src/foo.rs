// * enums give you a way of saying a value is one of a possible set of values
// * in this case an Ip address can be either version 4 or version 6 but not both

enum IpAddr {
    V4(String),
    V6(String),
}

pub fn hello() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
