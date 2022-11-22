#[allow(unused)]
pub fn demo_it() {
    if_let_basic();
    match_macro_demo();
    variable_override_if_let();
    variable_override_in_match();
}

fn if_let_basic() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Cannot match to a number");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {}", i);
    } else if i_like_letters {
        println!("Didn't match a number. but I like letters");
    } else {
        println!("Didn't match a number.");
    };
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn match_macro_demo() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // try filter, keep Foo
    // the following not work, since cannot compare enum
    // let filtered = v.iter().filter(|x| x == MyEnum::Foo);
    let filtered = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}", filtered);

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}

fn variable_override_if_let() {
    let age = Some(30);
    println!("before match age={:?}", age);

    if let Some(age) = age {
        println!("matched age={}", age);
    }

    println!("after match, age={:?}", age);
}

fn variable_override_in_match() {
    let age = Some(30);
    println!("before match, age={:?}", age);
    match  age {
        Some(age) => println!("matched, age={}", age),
        _ => (),
    }
    println!("after match, age={:?}", age);
}