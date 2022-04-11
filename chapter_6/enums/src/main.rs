//Defines an enum with two types
enum IpAddrKind {
    V4,
    V6,
}

//Structs could be used with enums:
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//But they can also contain data within their variants
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Currency {
    CAD,
    USD,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(Currency),
}

fn main() {
    //To create an instance of an enum use
    //double colon syntax
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    //Structs with IP addr
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddrKind2::V4(String::from("127.0.0.1"));

    //Rust does not have null values, but does contain option
    //Option is an enum of None or Some<T> which can be used to
    //Refer to a value or the absence of one similar to null

    let some_number = Some(5);
    let some_string = Some("The String");

    let absent_number: Option<i32> = None;

    //The rust compiler will make sure that you convert
    //Some values into a concrete value before performing operations on it
    //This is to prevent null/not null errors
    //This looks like in practice having code to handle each option that is
    //possible - Match can be used for this
}

//Enum types can be the parameter type to accept any instance
fn route(ip_kind: IpAddrKind) {}

//Ex of using match with an Enum
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luck Penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(currency) => {
            println!("This is a {:?} quarter", currency);
            25
        }
    }
}

//Ex of using if let instead of match to achieve something similar
fn value_in_cents2(coin: Coin) -> u32 {
    let mut count = 0;

    if let Coin::Quarter(currency) = coin {
        println!("This is a {:?} quarter", currency);
    } else {
        count += 1;
    };

    count
}
