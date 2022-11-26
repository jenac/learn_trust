#[allow(unused)]
// use std::convert::TryInto;
use std::ops::Add;
pub fn demo_it() {
    need_import_trait();
    demo_customized_add_op();
}

fn need_import_trait() {
    let a = 10;
    let b = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Then is less than one hundred.");
    }
}

#[derive(Debug)]
//trait defined in std::ops::Add
struct Point<T: Add<T, Output = T>> {
    //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    //implement trait for Point
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    //generic function now can be applied to Point
    a + b
}

fn demo_customized_add_op() {
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
}
