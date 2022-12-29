pub fn demo_it() {
    demo_mpsc();
    println!("----------");
    demo_try_recv();
    println!("----------");
    demo_send_data_w_owner();
    println!("----------");
    demo_send_with_for();
    println!("----------");
    demo_multiple_sender();
    println!("----------");
    demo_async_channel();
    println!("----------");
    demo_sync_channel();
    println!("----------");
    demo_sync_channel_2();
    println!("----------");
    demo_send_multiple_types();
}

// 其中mpsc是multiple producer, single consumer的缩写
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

fn demo_mpsc() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}

fn demo_try_recv() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
}

fn demo_send_data_w_owner() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("Hi, I am flying");
        tx.send(s).unwrap();
        // println!("val is {}", s);//error, s borrowed
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
}

use std::time::Duration;
fn demo_send_with_for() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received);
    }
}

fn demo_multiple_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("hi, from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi, from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got {}", received);
    }
}

//消息顺序
// 同步和异步通道
// Rust 标准库的mpsc通道其实分为两种类型：同步和异步。
fn demo_async_channel() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

fn demo_sync_channel() {
    let (tx, rx)= mpsc::sync_channel(0); //sync_channel, 0 缓存消息条数

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

fn demo_sync_channel_2() {
    let (tx, rx)= mpsc::sync_channel(1); //sync_channel, 0 缓存消息条数

    let handle = thread::spawn(move || {
        println!("首次发送之前");
        tx.send(1).unwrap();
        println!("首次发送之后");
        tx.send(2).unwrap();
        println!("再次发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

enum Fruit {
    Apple(u8),
    Orange(String),
}

fn demo_send_multiple_types() {
    let (tx, rx):(Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("good".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();
    
    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("Apple No. {}", count),
            Fruit::Orange(name) => println!("Orange named: {}", name),
        }
    }

}