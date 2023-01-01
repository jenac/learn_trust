use std::{sync::{Mutex, Arc}, thread};

#[allow(unused)]
pub fn demo_it() {
    demo_mutex_single_thread();
    println!("----------");
    demo_dead_lock();
    println!("----------");
    demo_mutex_in_threads();
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
