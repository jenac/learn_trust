pub fn demo_it() {
    demo_simple_box();
    demo_stack_heap();
    demo_box_trait_object();
    demo_vec_box();
    demo_box_leak();
}

/*堆栈的性能
很多人可能会觉得栈的性能肯定比堆高，其实未必。 由于我们在后面的性能专题会专门讲解堆栈的性能问题，因此这里就大概给出结论：

小型数据，在栈上的分配性能和读取性能都要比堆上高
中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
大型数据，只建议在堆上分配和使用

Box 的使用场景

特意的将数据分配在堆上
数据较大时，又不想在转移所有权时进行数据拷贝
类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
*/

fn demo_simple_box() {
    let a = Box::new(3);
    println!("a={}", a);

    // let b=a+1; // cannot add `{integer}` to `Box<{integer}>`
    let b = *a + 1;
    println!("b={}", b)
}

fn demo_stack_heap() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

// trait objects
trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("This is button[id={}]", self.id);
    }
}

struct Select {
    id : u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("This is select[id={}]", self.id);
    }
}

fn demo_box_trait_object() {
    let elems: Vec<Box<dyn Draw>> = vec![ 
        Box::new(Select{id: 12}), 
        Box::new(Button{id: 22}), 
        Box::new(Select{id: 32}), 
        ];
    for e in elems {
        e.draw();
    }
}

fn demo_vec_box() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + ** second;
    println!("sum = {}", sum);
}
/*
以上代码有几个值得注意的点：

使用 & 借用数组中的元素，否则会报所有权错误
表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，
第一次将 &Box<i32> 类型转成 Box<i32>，第二次将 Box<i32> 转成 i32
*/

fn demo_box_leak() {
    let s = gen_static_str();
    println!("generated static str: {}", s);
}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("Hello, this will leaked to be static");

    Box::leak(s.into_boxed_str())
}
/*Box 中还提供了一个非常有用的关联函数：Box::leak，
它可以消费掉 Box 并且强制目标值从内存中泄漏，读者可能会觉得，这有啥用啊？

其实还真有点用，例如，你可以把一个 String 类型，
变成一个 'static 生命周期的 &str 类型：
 */