/*
什么是字符串?
顾名思义，字符串是由字符组成的连续集合，但是在上一节中我们提到过，Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间。

Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。虽然语言级别只有上述的 str 类型，但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。

str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。
*/
#![allow(unused)]
pub fn demo_it() {
    push_demo();
    insert_demo();
    replace_demo();
    replacen_demo();
    replace_range_demo();
    pop_demo();
    remove_demo();
    truncate_demo();
    clear_demo();
}

fn push_demo() {
    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符 push() -> {}", s);

    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);
}

fn insert_demo() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}

fn replace_demo() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
}

fn replacen_demo() {
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
}

fn replace_range_demo() {
    //inplace replace
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..9, "R");
    dbg!(string_replace_range);
}

/*pop —— 删除并返回字符串的最后一个字符

该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None。 示例代码如下：
*/
fn pop_demo() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

/*remove —— 删除并返回字符串中指定位置的字符

该方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
*/
fn remove_demo() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    let r = string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    let s = string_remove.remove(3);
    dbg!(string_remove);
    dbg!(r);
    dbg!(s);
}

/*truncate —— 删除字符串中从指定位置开始到结尾的全部字符

该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
*/

fn truncate_demo() {
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3); // include 3
    dbg!(string_truncate);
}

/*
clear —— 清空字符串

该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
*/
fn clear_demo() {
    let mut string_clear = String::from("string to clear");
    string_clear.clear();
    dbg!(string_clear);
}
