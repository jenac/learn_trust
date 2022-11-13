#![allow(unused)]
pub fn demo_it() {
    string_slice();
    more_string_slice();
    invalid_range();
    mut_immut_again();
}

fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello={}, world={}", hello, world)
}

fn more_string_slice() {
    let s = String::from("hello");

    let slice0 = &s[0..2];
    let slice1 = &s[..2];
    println!("{}={}", slice0, slice1);
    let len = s.len();

    let slice2 = &s[4..len];
    let slice3 = &s[4..];
    println!("{}={}", slice2, slice3);

    let slice4 = &s[0..len];
    let slice5 = &s[..];
    println!("{}={}", slice4, slice5);
}

fn invalid_range() {
    /*
    在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，
    也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：
    */
    let s = "中国人";
    // let a = &s[0..2]; // error
    let a = &s[0..3]; // good
    println!("{}", a);
}

fn mut_immut_again() {
    let mut s = String::from("hello world");
    let word = first_word(&s); //immutatble ref
                               // s.clear(); // error! mut borrow cannot coexist with immutable ref
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}
