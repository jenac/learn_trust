pub fn demo_it() {
    demo_avoid_override_builtin();
    demo_better_readable_code();
    demo_hide_power();
}

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}
fn demo_avoid_override_builtin() {
    let w = Wrapper(vec![
        "hello".to_string(),
        "word".to_string(),
        "again".to_string(),
        "?".to_string(),
    ]);
    println!("w={}", w);
}

//more readable code
use std::ops::Add;

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "目标地点距离你{}米", self.0)
    }
}
impl Add for Meters{
    type Output=Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

fn demo_better_readable_code() {
    let d = calculate_distance(Meters(2), Meters(10));
    println!("{}", d);
}

fn calculate_distance(meters_1: Meters, meters_2: Meters) -> Meters {
    meters_1 + meters_2
}

//hide 

struct HidePower(u32);

fn demo_hide_power() {
    let i:u32 = 2;
    assert_eq!(i.pow(2), 2);

    let _n = HidePower(i);
    // 下面的代码将报错，因为`HidePower`类型上没有`pow`方法
    // assert_eq!(n.pow(2), 4);
}

//Alias
#[allow(dead_code)]
type AliasMeters = u32;

/*
类型别名仅仅是别名，只是为了让可读性更好，并不是全新的类型，newtype 才是！
类型别名无法实现为外部类型实现外部特征等功能，而 newtype 可以
*/

