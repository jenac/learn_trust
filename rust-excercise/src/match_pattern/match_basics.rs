#![allow(unused)]
pub fn demo_it() {
    match_demo();
    match_value_demo();
    match_is_expression();
    print_state_for_quarter();
    match_actions();
    match_default();

}

enum Direction {
    East,
    West,
    North,
    South,
}

fn match_demo() {
    let dir = Direction::South;
    match dir {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("N or S");
        }
        _ => println!("West"),
    };
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match  coin {
        Coin::Penny => {
            println!("Luck penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter =>25,
    }
}

fn match_value_demo() {
    println!("{:?} = {}", Coin::Penny, value_in_cents(Coin::Penny));
    println!("{:?} = {}", Coin::Nickel, value_in_cents(Coin::Nickel));
    println!("{:?} = {}", Coin::Dime, value_in_cents(Coin::Dime));
    println!("{:?} = {}", Coin::Quarter, value_in_cents(Coin::Quarter));
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

fn match_is_expression() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
}

#[derive(Debug)]
enum UsState {
    Minnesota,
    Alaska,
    Alabama,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn print_state_for_quarter() {
    let c = Coin2::Quarter(UsState::Minnesota);
    let v = match c {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    };
    println!("v={}", v);
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(i32, i32, i32),
}

fn match_actions() {
    let actions = [
        Action::Say("Hello, Rust!".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("Moving to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, b) => {
                println!("Change color: r={}, g={}, b={}", r, g, b);
            },
        }
    }
}

fn match_default() {
    let some_u8 = 0u8;
    match some_u8 {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
}

