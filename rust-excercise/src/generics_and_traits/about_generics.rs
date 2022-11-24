#![allow(unused)]
pub fn demo_it() {
    add_demo();
    use_t_in_struct();
    t_in_method();
    t_in_method2();
    impl_concret_only();
    demo_const_generic();
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn add_demo() {
    println!("add i8: {}", add(2i8, 11i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23f64, 2.23));
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}
fn use_t_in_struct() {
    let i = Point { x: 1, y: 2 };
    let f = Point { x: 1.2, y: 2.3 };

    let k = Point2 { x: 1, y: 1.2 };
    let l = Point2 { x: 1, y: 2 };
}

//generic in enum
/*
enum Option<T> {
    Some(T),
    None,
}
*/

//generic in method
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn t_in_method() {
    let p = Point { x: 12, y: 18 };
    println!("p.x={}", p.x());
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn t_in_method2() {
    let p1 = Point2 { x: 5, y: 1.2 };
    let p2 = Point2 { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);

    // println!("p1=({}, {})", p1.x, p1.y); // p1 borrowed can't be accessed.
    // println!("p2=({}, {})", p2.x, p2.y); // p2 borrowed can't be accessed.
    println!("p3=({},{})", p3.x, p3.y);
}

//implement concret only
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn impl_concret_only() {
    let p = Point { x: 3.0f32, y: 4.0 };
    println!("p from origin is {}", p.distance_from_origin());
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn demo_const_generic() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
