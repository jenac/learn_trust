// use std::ops::{Range, RangeInclusive};
// fn main() {
  // for c in 'a'..='z' {
  //     println!("{}",c as u8);
  // }

  // for i in 1..5 {
  //   println!("{}", i)
  // }

  // for i in 1..=5 {
  //   println!("{}", i)
  // }

  // print!("{}", 9.6 / 3.2)
  
  // let x = '中';
  // println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));


  // let f = false;
  // let t = true && false;
  // print!("{}", t.to_string());
    
  // assert_eq!(t, f);

  // println!("Success!");

  // let unit: () = ();
  // assert!(std::mem::size_of_val(&unit) == 0);

  // println!("Success!")

  // assert_eq!(ret_unit_type(), ());

  // never_return();
// }

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    println!("{}", y);
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };
    println!("{}", _z)
  
}

use std::thread;
use std::time;

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    loop {
        println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1))
    }
}

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!()
// }
// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     todo!();
// }
// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     loop {
//         std::thread::sleep(std::time::Duration::from_secs(1))
//     }
// }