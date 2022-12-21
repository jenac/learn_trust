#[allow(unused)]
pub fn demo_it() {
    demo_basic_deref();
    demo_smart_pointer_deref();
    demo_impl_deref();
    demo_continuous_implict_deref();
    demo_flat_ref();
    demo_impl_mut_deref();
}

fn demo_basic_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    /*
    这里 y 就是一个常规引用，包含了值 5 所在的内存地址，然后通过解引用 *y，我们获取到了值 5
    */
}

fn demo_smart_pointer_deref() {
    let x=Box::new(123);
    let sum = *x + 12;
    println!("sum={}", sum);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn demo_impl_deref() {
    let y = MyBox::new(12);
    assert_eq!(12, *y); //need impl deref, otherwise error
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
当我们对智能指针 Box 进行解引用时，实际上 Rust 为我们调用了以下方法：
*(y.deref())
首先调用 deref 方法返回值的常规引用，然后通过 * 对常规引用进行解引用，最终获取到目标值。
*/

fn demo_continuous_implict_deref() {
    let s = MyBox::new(String::from("I am good"));
    display(&s);
}

fn display(s: &str) {
    println!("{}",s);
}
/*
这里我们使用了之前自定义的智能指针 MyBox，并将其通过连续的隐式转换变成 &str 类型：
首先 MyBox 被 Deref 成 String 类型，结果并不能满足 display 函数参数的要求，
编译器发现 String 还可以继续 Deref 成 &str，最终成功的匹配了函数参数。
*/

struct Foo;

impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}

fn demo_flat_ref() {
    let f = &&Foo;
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}

/*
三种 Deref 转换
在之前，我们讲的都是不可变的 Deref 转换，
实际上 Rust 还支持将一个可变的引用转换成另一个可变的引用以及将一个可变引用转换成不可变的引用，
规则如下：

当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
*/

struct MyBox1<T> {
    v: T,
}

impl<T> MyBox1<T> {
    fn new(x:T) -> MyBox1<T> {
        MyBox1 { v: x }
    }
}

impl<T> Deref for MyBox1<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for MyBox1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn demo_impl_mut_deref() {
    let mut s = MyBox1::new(String::from("heloo, K"));

    display1(&mut s);
}

fn display1(s: &mut String)  {
    s.push_str("What are you doing");
    println!("{}", s);
}

/*
要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
T: DerefMut<Target=U> 解读：将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，
对应上例中，就是将 &mut MyBox<String> 转换为 &mut String
*/