#[allow(unused)]
pub fn demo_it() {
    option_demo();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn option_demo() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five={:?}, six={:?}, none={:?}", five, six, none);
}
