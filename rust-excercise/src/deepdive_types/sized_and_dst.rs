#[allow(unused)]
pub fn demo_it() {
    demo_about_str();
    demo_box_str();
}

fn demo_about_str() {
    // error, compile does not know the size of s1, s2
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // ok, s3 is a ptr/ref, with size info 
    /*
    那么为何字符串切片 &str 就是固定大小呢？因为它的引用存储在栈上，具有固定大小(类似指针)，
    同时它指向的数据存储在堆中，也是已知的大小，再加上 &str 引用中包含有堆上数据内存地址、
    长度等信息，因此最终可以得出字符串切片是固定大小类型的结论。
    */
    let _s3: &str = "on?";
}


// fn foobar_1(thing: &dyn MyThing) {}     // OK
// fn foobar_2(thing: Box<dyn MyThing2>) {} // OK
// fn foobar_3(thing: MyThing3) {}          // ERROR!
// 如上所示，只能通过引用或 Box 的方式来使用特征对象，直接使用将报错！


fn demo_box_str() {
    // error:  ^^^^^^^^ doesn't have a size known at compile-time
    // let s1: Box<str> = Box::new("Hello there!" as str);

    //this works:
    let _s1: Box<str> = "Hello there!".into();

}