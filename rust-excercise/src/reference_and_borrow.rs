#![allow(unused)]

pub fn demo_it() {
    ref_and_deref();
    immutable_ref();
    mutable_ref();
    scope_resolve_above();
    compiler_optimization();
  
}

fn ref_and_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn immutable_ref() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//正如变量默认不可变一样，引用指向的值默认也是不可变的，没事，来一起看看如何解决这个问题。
#[allow(dead_code)]
fn change(_some_string: &String) {
    // some_string.push_str(", world"); //error here
}

fn mutable_ref() {
    let mut s = String::from("hello");
    change_again(&mut s);
    println!("change_again '{}'", s);
}

fn change_again(some_string: &mut String) {
    some_string.push_str(", world");
}

//可变引用同时只能存在一个

fn only_one_mut_ref() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; //error
    // println!("{}, {}", r1, r2);
}

fn scope_resolve_above() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}

//可变引用与不可变引用不能同时存在
fn immut_mut_ref_non_coexist() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
                 // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);
}

//Rust 的编译器一直在优化
fn compiler_optimization() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束


// NLL
// 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，

//悬垂引用(Dangling References)

fn dangle_demo() {
  // let reference_to_nothing = dangle();
  let reference_to_nothing = no_dangle();
}
// fn dangle() -> &String { // dangle 返回一个字符串的引用

    // let s = String::from("hello"); // s 是一个新字符串

    // &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// 借用规则总结
// 总的来说，借用规则如下：

// 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
// 引用必须总是有效的