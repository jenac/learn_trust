#![allow(unused)]
pub fn demo_it() {
    destruct_matching();
    using_dot();
  return_tup();
}

fn destruct_matching() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z)
}

fn using_dot() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x={}, y={}, z={}", x, y, z)
}

// calculate_length 函数接收 s1 字符串的所有权，然后计算字符串的长度，
// 接着把字符串所有权和字符串长度再返回给 s2 和 len 变量。
fn return_tup() {
  let s1 = String::from("hello");
  let (s2, len) = calculate_length(s1);
  println!("len of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length=s.len();
  return (s, length);
}