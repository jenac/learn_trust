#![allow(unused)]

pub fn demo_it() {
  var_scope();
  string_intro();
  ownership_change();
  reference();
  clone_deep_copy();
  some_demo();
  partial_move();
}

pub fn var_scope() {
  let s = "hello";
}


// 字符串字面值是不可变的，因为被硬编码到程序代码中
// 并非所有字符串的值都能在编写代码时得知


pub fn string_intro() {
  let s1 = String::from("hello");
  
  let mut s = String::from("hello");
  s.push_str(", world!"); // push_str() 在字符串后追加字面值
  println!("{}", s); // 将打印 `hello, world!`
}


pub fn ownership_change() {
  let s1 = String::from("hello");
  let s2 = s1;

  // println!("{}, world!", s1); //error here
}

pub fn reference() {
  let x: &str = "hello, world"; //x is a reference, does not have ownership
  let y = x;
  println!("{},{}",x,y);
  let z = y;
  println!("{}", z);
}

pub fn clone_deep_copy() {
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);
}

/*
Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。

那么什么类型是可 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则： 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

所有整数类型，比如 u32。
布尔类型，bool，它的值是 true 和 false。
所有浮点数类型，比如 f64。
字符类型，char。
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的
*/

pub fn some_demo() {
  let s1 = gives_ownership();
  // gives_ownership 将返回值移给 s1
  println!("s1={}", s1);

  let s2 = String::from("hello, s2");
  // s2 进入作用域

  let s3 = takes_and_gives_back(s2);
  // s2 被移动到 takes_and_gives_back 中,
  // 它也将返回值移给 s3
  println!("s3={}", s3);
  // println!("s2={}", s2); //error
}

fn gives_ownership() -> String {             
  // gives_ownership 将返回值移动给
  // 调用它的函数
  let some_string = String::from("hello"); 
  // some_string 进入作用域.
  some_string
  // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {   // a_string 进入作用域
  a_string  // 返回 a_string 并移出给调用的函数
}

pub fn partial_move() {
   #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}