pub fn demo_it() {
    demo_atomic_add_n_times();
    demo_mem_barrier();
    demo_atomic_in_multi_threads();
}

use std::{
    ops::Sub,
    sync::{atomic::{AtomicU64, Ordering, AtomicBool, AtomicUsize}, Arc},
    thread::{self, JoinHandle},
    time::Instant, hint,
};

const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn demo_atomic_add_n_times() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("{:?}", Instant::now().sub(s));
}
/*
限定内存顺序的 5 个规则
在理解了内存顺序可能存在的改变后，你就可以明白为什么 Rust 提供了Ordering::Relaxed用于限定
内存顺序了，事实上，该枚举有 5 个成员:

Relaxed， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
Release 释放，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，但是它后面的操作
    可能被重排到它前面
Acquire 获取, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之前的操作却有可能被重排到
    它后面，往往和Release在不同线程中联合使用
AcqRel, 是 Acquire 和 Release 的结合，同时拥有它们俩提供的保证。比如你要对一个 atomic 
    自增 1，同时希望该操作之前和之后的读取或写入操作不会被重新排序
SeqCst 顺序一致性， SeqCst就像是AcqRel的加强版，它不管原子操作是属于读取还是写入的操作，
    只要某个线程有用到SeqCst的原子操作，线程中该SeqCst操作前的数据操作绝对不会被重新排在
    该SeqCst操作之后，且该SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前。
*/

static mut DATA: u64=0;
static READY: AtomicBool = AtomicBool::new(false);

fn demo_mem_barrier() {
    reset();

    let t_producer = producer();
    let t_consumer = consumer();

    t_producer.join().unwrap();
    t_consumer.join().unwrap();
}

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA=100; //A
        }
        READY.store(true, Ordering::Release);  // B: 内存屏障 ↑
    })
}   

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        // C: 内存屏障 ↓
        while !READY.load(Ordering::Acquire)  {
            assert_eq!(100, unsafe {
                DATA //D
            });
        }  
    })
}

fn demo_atomic_in_multi_threads() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}