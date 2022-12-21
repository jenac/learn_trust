#[allow(unused)]
pub fn demo_it() {
    demo_dropping_basics();
    demo_manual_drop();
}

struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2");
    }
}

#[allow(dead_code)]
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops")
    }
}
struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo");
    }
}

fn demo_dropping_basics() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };

    let _foo = Foo;
    println!("Running");
}

fn demo_manual_drop() {
    let f = Foo;
    drop(f);
    // f.drop(); 
    // println!("Running!:{:?}", foo);
}

/*
互斥的 Copy 和 Drop
我们无法为一个类型同时实现 Copy 和 Drop 特征。因为实现了 Copy 的特征会被编译器隐式的复制，
因此非常难以预测析构函数执行的时间和频率。因此这些实现了 Copy 的类型无法拥有析构函数。
*/