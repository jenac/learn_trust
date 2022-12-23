#[allow(unused)]
pub fn demo_it() {
    demo_cell();
    // demo_ref_cell();
    demo_ref_cell_usage();
    demo_rc_plus_refcell();
    demo_retain_even();
}

/*
内部可变性的实现是因为 Rust 使用了 unsafe 来做到这一点，
但是对于使用者来说，这些都是透明的，因为这些不安全代码都被封装到了安全的 API 中

Cell
Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况：


代码展示了 Cell 的基本用法，有几点值得注意：
"asdf" 是 &str 类型，它实现了 Copy 特征
c.get 用来取值，c.set 用来设置新值
*/

use std::cell::Cell;
fn demo_cell() {
    let c= Cell::new("asdf");
    let one = c.get();
    c.set("qwera");
    let two = c.get();
    println!("{}, {}", one, two);
}

/*
string need use refcell
compile pass but run time error/panic
*/
use std::cell::RefCell;
#[allow(dead_code)]
fn demo_ref_cell() {
    let s = RefCell::new(String::from("good buy"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();
    println!("{}, {}", s1, s2);
}

// 定义在外部库中的特征
pub trait Messenger {
    fn send(&self, msg: String);
}

// --------------------------
// 我们的代码中的数据结构和实现
// struct MsgQueue {
//     msg_cache: Vec<String>,
// }

// impl Messenger for MsgQueue {
//     fn send(&self, msg: String) {
//         self.msg_cache.push(msg) 
//         //error, and we cannot change to send(&mut self, msg: String)
//     }
// }

pub struct  MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}

fn demo_ref_cell_usage() {
    let mq = MsgQueue{
        msg_cache: RefCell::new(Vec::new()),
    };

    mq.send("hello".to_string());
}
/*
Rc + RefCell
*/
use std::rc::Rc;
fn demo_rc_plus_refcell() {
    let s = Rc::new(
        RefCell::new("我很善变，还拥有多个主人".to_string()));
    println!("before: {:?}", s);
    let s1 = s.clone();
    let s2 = s.clone();
    s2.borrow_mut().push_str(", oh yeah!");
    println!("after: {:?}\n{:?}\n{:?}", s, s1, s2);
}

fn demo_retain_even() {
    let mut nums = vec![1,2,3,4,5,6,7,8,9,0];
    retain_event(&mut nums);
    println!("{:?}", nums)
}
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
fn retain_event(nums: &mut Vec<i32>){
    let slice = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    let mut i = 0;
    for num in slice
        .iter()
        .filter(|num| is_even(num.get())) {
            slice[i].set(num.get());
            i+=1;
        }
    nums.truncate(i);
}