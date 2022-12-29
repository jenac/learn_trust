#[allow(unused)]
pub fn demo_it() {
    demo_thread();
    println!("----------");
    demo_thread_w_move();
    println!("----------");
    demo_barrier();
    println!("----------");
    demo_thread_local();
    println!("----------");
    demo_mutex_condvar();
    println!("----------");
    demo_only_once();
}

use std::thread;
use std::time::Duration;

fn demo_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn demo_thread_w_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // 下面代码会报错borrow of moved value: `v`
    // println!("{:?}",v);
}

// 线程屏障(Barrier)
// 在 Rust 中，可以使用 Barrier 让多个线程都执行到某个点后，才继续一起往后执行：

use std::sync::{Arc, Barrier};

fn demo_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    // let barrier = Arc::new(Barrier::new(3));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

// 标准库 thread_local
// 使用 thread_local 宏可以初始化线程局部变量，然后在线程内部使用该变量的 with 方法获取变量值：

use std::cell::RefCell;

fn demo_thread_local() {
    thread_local! (static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // 等待线程完成
    t.join().unwrap();

    // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

//用条件控制线程的挂起和执行
//条件变量(Condition Variables)经常和 Mutex 一起使用，可以让线程挂起，直到某个条件发生后再继续执行：
use std::sync::{Condvar, Mutex};

fn demo_mutex_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

use std::sync::Once;

static mut VAL:usize = 0;
static INIT: Once = Once::new();

fn demo_only_once() {

    let handle1 = thread::spawn(move || {
        INIT.call_once(||{
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(||{
            unsafe {
                VAL = 2;
            }
        });
    });
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });

}
