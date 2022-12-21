#[allow(unused)]
pub fn demo_it() {
    demo_rc();
    demo_arc();
}

use std::rc::Rc;

struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn demo_rc() {
    // 创建一个基于引用计数的 `Owner`.
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });

    // 创建两个不同的工具，它们属于同一个主人
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // 释放掉第一个 `Rc<Owner>`
    drop(gadget_owner);

    // 尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
    // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
    // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
    // 引用指向底层的 owner 数据，引用计数尚未清零
    // 因此 owner 数据依然可以被使用
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
    // 数据也被清理释放
}

/*
Rc 简单总结
1.  Rc/Arc 是不可变引用，你无法修改它指向的值，只能进行读取，
    如果要修改，需要配合后面章节的内部可变性 RefCell 或互斥锁 Mutex
2.  一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
3.  Rc 只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 Arc
4.  Rc<T> 是一个智能指针，实现了 Deref 特征，因此你无需先解开 Rc 指针，再使用里面的 T，
    而是可以直接使用 T，例如上例中的 gadget1.owner.name
 */

/*
 Arc
Arc 是 Atomic Rc 的缩写，顾名思义：原子化的 Rc<T> 智能指针。
*/
use std::sync::Arc;
use std::thread;

fn demo_arc() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..20 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("{}", s);
        });
        let _ = handle.join();
    }
}
