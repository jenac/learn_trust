#[allow(unused)]
pub fn demo_it() {
    demo_vector();
    demo_borrow();
    demo_traversal();
    demo_with_enum();
    demo_with_trait_object();
}

fn demo_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("{:?}", v);

    let mut v2 = vec![1, 2, 3, 4];
    v2.push(100);
    println!("{:?}", v2);

    let third = &v2[2]; //panic if index out fo bounds
    println!("the third element in v2 = {}", third);

    match v2.get(2) {
        Some(e3) => println!("3rd element in v2 is {}", e3),
        None => println!("are you sure we have 3rd element?"),
    }
}

#[allow(unused)]
fn demo_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // the follow will error, since v.push has mutable borrow
    // println!("the first element is {}", first);
}

fn demo_traversal() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i)
    }

    //mutable traversal
    let mut c = vec![1, 2, 3, 4, 5];
    for i in &mut c {
        *i += 10
    }
    println!("{:?}", c);
}

fn demo_with_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        println!("{:?}", ip);
    }
}

fn demo_with_trait_object() {
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("IPV4: {:?}", self.0);
        }
    }

    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("V6: {:?}", self.0);
        }
    }

    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("192.168.1.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
