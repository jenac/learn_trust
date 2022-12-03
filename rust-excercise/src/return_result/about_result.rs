use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

pub fn demo_it() {
    demo_basic_result();
    demo_question_mark_macro();
    demo_with_multiply();
    demo_and_then();
    demo_with_multiply2();
}

fn demo_basic_result() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file:{:?}", other_error),
        },
    };

    //unwrap will panic if fail
    // let _f2 = File::open("not_exist.txt").unwrap();

    //expect will panic if fail but with customized package
    // let _f3 = File::open("not_exits.txt").expect("What!!!!!");
}

fn demo_question_mark_macro() {
    let s = read_string_from_file();
    println!("{:?}", s);

    let p = read_string_from_file_v2();
    println!("{:?}", p);
    println!("{:?}", read_string_from_file_v3());
}

fn read_string_from_file() -> Result<String, io::Error> {
    let mut f = File::open("not_exists.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_string_from_file_v2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("not_exists.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
use std::fs;
fn read_string_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("not_exists.txt")
}

use std::num::ParseIntError;

fn multiply_unwrap(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn multiply_wit_question_mark(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn demo_with_multiply() {
    let result = multiply_unwrap("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply_unwrap("4", "2");
    assert_eq!(result.unwrap(), 8);

    assert_eq!(multiply_wit_question_mark("3", "4").unwrap(), 12);

    println!("Success!");
}

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn demo_and_then() {
    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success!");
}

type Res<T> = Result<T, ParseIntError>;
fn multiply_with_and_then_map(n1_str: &str, n2_str: &str) -> Res<i32> {
    n1_str
        .parse::<i32>()
        .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
}

fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n={}", n),
        Err(e) => print!("err={}", e),
    }
}
fn demo_with_multiply2() {
    let twenty = multiply_with_and_then_map("10", "2");
    print(twenty);

    // 下面的调用会提供更有帮助的错误信息
    let tt = multiply_with_and_then_map("t", "2");
    print(tt);

    println!("Success!")
}
