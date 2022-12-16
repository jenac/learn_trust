use std::vec;

#[allow(unused)]
pub fn demo_it() {
    demo_basic();
    demo_struct_with_closure();
    demo_fn_once();
    demo_closure_with_move();
    demo_mut_fn();
    demo_with_fnmut();
    demo_return_closure();
}

fn demo_basic() {
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
}

fn demo_struct_with_closure() {
    let t = |x: u32| x + 2;
    let mut c = Cacher::new(t);
    let x = c.value(100);
    println!("{}", x);

    let t2 = |x: f32| x + 0.2;
    let mut c2 = Cacher::new(t2);
    let x2 = c2.value(3.12159);
    println!("{}", x2);
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy,
{
    query: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy,
{
    fn new(query: T) -> Cacher<T, U> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn demo_fn_once() {
    let x = vec![1,2,3];
    fn_once(|z| z==x.len());
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy, //only call once, unless + Copy
{
    println!("{}", func(3));
    println!("{}", func(4));
}

/*
如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，
这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。
*/
fn demo_closure_with_move() {
    use std::thread;
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("here is the vector: {:?}", v);
    });
    handle.join().unwrap();
}

/*
FnMut，它以可变借用的方式捕获了环境中的值，因此可以修改该值
*/
fn demo_mut_fn() {
    let mut s = String::new();

    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);
}

fn demo_with_fnmut() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{:?}", s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

/*
实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：

所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
*/

fn factory(x:i32) -> Box<dyn Fn(i32)->i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x+num)
    } else {
        Box::new(move |x| x- num)
    }
}

fn demo_return_closure() {
    let f = factory(-1);
    println!("{:?}", f(100));
    let f2 = factory(2);
    println!("{:?}", f2(100));
}