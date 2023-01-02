use std::{sync::{Mutex, Arc, MutexGuard, RwLock, Condvar}, thread::{self, sleep}, time::Duration};

#[allow(unused)]
pub fn demo_it() {
    demo_mutex_single_thread();
    println!("----------");
    demo_dead_lock();
    println!("----------");
    demo_mutex_in_threads();
    println!("----------");
    // demo_deadlock_in_multiple_threads();
    println!("----------");
    demo_try_lock();
    println!("----------");
    demo_rw_lock();
    println!("----------");
    demo_condvar();
}


fn demo_mutex_single_thread() {
    // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);

    {
        // 获取锁，然后deref为`m`的引用
        // lock返回的是Result
        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁自动被drop
    }

    println!("m = {:?}", m);
}

/*
在注释中，已经大致描述了代码的功能，不过有一点需要注意：和Box类似，数据被Mutex所拥有，要访问内部的数据，需要使用方法m.lock()向m申请一个锁, 该方法会阻塞当前线程，直到获取到锁，因此当多个线程同时访问该数据时，只有一个线程能获取到锁，其它线程只能阻塞着等待，这样就保证了数据能被安全的修改！

m.lock()方法也有可能报错，例如当前正在持有锁的线程panic了。在这种情况下，其它线程不可能再获得锁，因此lock方法会返回一个错误。

这里你可能奇怪，m.lock明明返回一个锁，怎么就变成我们的num数值了？聪明的读者可能会想到智能指针，没错，因为Mutex<T>是一个智能指针，准确的说是m.lock()返回一个智能指针MutexGuard<T>:

它实现了Deref特征，会被自动解引用后获得一个引用类型，该引用指向Mutex内部的数据
它还实现了Drop特征，在超出作用域后，自动释放锁，以便其它线程能继续获取锁
正因为智能指针的使用，使得我们无需任何操作就能获取其中的数据。 如果释放锁，你需要做的仅仅是做好锁的作用域管理
*/

fn demo_dead_lock() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
    drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    drop(num1); // 手动 drop num1 ，观察打印结果的不同

    println!("m = {:?}", m);
}

fn demo_mutex_in_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num+=1;
            println!("now counter is {:?}", num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

//简单总结下：Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutex<T>用于多线程内部可变性。


use lazy_static::lazy_static;
lazy_static!{
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

#[allow(dead_code)]
fn demo_deadlock_in_multiple_threads() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let _guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let _guard = MUTEX2.lock().unwrap();
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}


fn demo_try_lock() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let _guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.try_lock();
                    println!("线程1获取MUTEX2锁的结果: {:?}",guard);
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    sleep(Duration::from_millis(10));
                    let guard = MUTEX1.try_lock();
                    println!("线程2获取MUTEX1锁的结果: {:?}",guard);
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}



fn demo_rw_lock() {
    let lock = RwLock::new(5);

    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 以下代码会panic，因为读和写不允许同时存在
        // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
        // let r1 = lock.read();
        // println!("{:?}",r1);
    }// 写锁在此处被drop
}


fn demo_condvar() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = thread::spawn(move || {
        let mut m = {*cflag.lock().unwrap()};
        let mut counter = 0;

        while counter < 3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }
            {
                m = false;
                *cflag.lock().unwrap() = true;
            }
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter+=1;
        if counter > 3 {
            println!("breaking - outside counter: {}", counter);
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }

    hdl.join().unwrap();
    println!("{:?}", flag);
}


