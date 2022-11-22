#[allow(unused)]

pub fn demo_it() {
    match_literal();
    match_variable();
    or_in_pattern();
    match_range();
    match_and_destruct_value();
    match_and_destruct_enum();
    match_nested_enum();
    destruct_struct_and_tuple();
    ignore_value();
    diff_underscore();
    ignore_with_double_dots();
    match_guard();
    at_binding();
}

//match literal
fn match_literal() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => print!("not one or two"),
    }
}

fn match_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("matched, y={:?}", y),
        _ => println!("default case, x={:?}", x),
    }

    println!("at the end, x = {:?}, y={:?}", x, y);
}

fn or_in_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => print!("others"),
    }
}

fn match_range() {
    let x = 5;
    match x {
        1..=5 => println!("1 to 5 here"),
        _ => println!("others"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("others"),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn match_and_destruct_value() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("not on axis, x={}, y={}", x, y),
    }
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn match_and_destruct_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("move x={}, y={}", x, y);
        }
        Message::Write(text) => {
            println!("write text={}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("change color r,g,b={},{},{}", r, g, b);
        }
    }
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn match_nested_enum() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 16, 25));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("color (r,g,b) == ({}, {}, {})", r, g, b);
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("color (h,s,v) == ({}, {}, {})", h, s, v);
        }
        _ => (),
    }
}

// struct Point {
//     x: i32,
//     y:i32,
// }
fn destruct_struct_and_tuple() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 18, y: -10 });
    println!("feet={}, inches={}, x={}, y={}", feet, inches, x, y);
}

fn ignore_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cann't overwrite an exisitng value")
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("1={}, 3={}, 5={}", first, third, fifth);
        }
    }
}

fn diff_underscore() {
    let s = Some(String::from("hello, world"));

    // if let Some(_s) = s {
    //     println!("s is borrowed")
    // }
    // println!("{:?}", s) // s is borrowed and cannot be accessed here

    if let Some(_) = s {
        println!("s is not borrowed, since _ does not do actual binding")
    }
    println!("{:?}", s)
}

#[allow(dead_code)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_with_double_dots() {
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => {
            println!("only care about x={}", x);
        }
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("first={}, last={}", first, last);
        }
    }
}

fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("same with y, n={}, y={}", n, y),
        _ => println!("x={:?}", x),
    }
    println!("after match, x={:?}, y={}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

enum Message3 {
    Hello { id: i32 },
}
fn at_binding() {
    let msg = Message3::Hello { id: 3 };

    match msg {
        Message3::Hello { id: id_var @ 3..=7 } => {
            println!("found in range: {}", id_var);
        },
        Message3::Hello { id: 10..=12 } => {
            println!("id in range 10 to 12");
        },
        Message3::Hello { id: _ } => {
            println!("others");
        }
    }

    //
    let p@Point{x: px, y: py} = Point{x:10, y:23};
    println!("x={}, y={}", px, py);
    println!("{:?}", p);

    let point = Point{x:10, y:5};
    if let p@Point { x:10, y } = point {
        println!("x=10 and y={} in {:?}", y, p);
    } else {
        println!("x!=10");
    }

    //new to 1.53
    match 1 {
        num @(1|2) => {
            println!("{}", num);
        }
        _=> {}
    }
}
