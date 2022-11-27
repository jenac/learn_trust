#[allow(unused)]
pub fn demo_it() {
    demo_add_same_type();
    demo_add_diff_types();
    demo_call_same_method_name();
    demo_call_associate_method_with_same_name();
    demo_trait_constrain();
    demo_new_type();
}

//Add define in std:
/*
trait Add<RHS=Self> { //defaut value generic type
    type Output; //associate type

    fn add(self, rhs: RHS) -> Self::Output;
}
*/

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn demo_add_same_type() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

fn demo_add_diff_types() {
    let m = Meters(2);
    let mm = Millimeters(345);
    println!("mm+m={:?}mm", mm + m);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("flying airbus A380")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up@@")
    }
}

impl Human {
    fn fly(&self) {
        println!("I believe I can fly...")
    }
}

fn demo_call_same_method_name() {
    let p = Human {};
    p.fly();
    Pilot::fly(&p);
    Wizard::fly(&p);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

fn demo_call_associate_method_with_same_name() {
    println!("Baby dog name - {}", Dog::baby_name());
    println!(
        "Baby dog name (from Animal) - {}",
        <Dog as Animal>::baby_name()
    );
}

//特征定义中的特征约束
//如果你想要实现 OutlinePrint 特征，首先你需要实现 Display 特征。
use std::fmt::Display;

trait OutlinPrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinPrint for Point {}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn demo_trait_constrain() {
    let p = Point { x: 10, y: 29 };
    p.outline_print();
}

//newtype,
// to avoid the following
/*
impl<T> std::fmt::Display for Vec<T> {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------
| |                             |
| |                             Vec is not defined in the current crate
*/

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn demo_new_type() {
    let w = Wrapper(vec!["hello".to_string(), String::from("world")]);
    println!("w={}", w);
}
