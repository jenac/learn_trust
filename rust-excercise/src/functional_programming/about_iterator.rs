pub fn demo_it() {
    demo_basics();
    demo_iter_lazy();
    demo_next();
    demo_impl_for();
    demo_3_inters();
    demo_consumer_sum();
    demo_collect();
    demmo_filter();
    demo_implement_iterator();
    demo_others();
}

/*
简而言之就是数组实现了 IntoIterator 特征，Rust 通过 for 语法糖，自动把实现了该特征的数组类型转换为迭代器（你也可以为自己的集合类型实现此特征）
 */

fn demo_basics() {
    let arr = [1, 2, 5];
    for v in arr {
        println!("{}", v);
    }

    println!("----------");
    for i in 1..10 {
        println!("{}", i);
    }

    println!("----------");
    for v in arr.into_iter() {
        println!("{}", v)
    }
}

//iterator is lazy, if it is not used, then do nothing
fn demo_iter_lazy() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    println!("----------");
    for val in v1_iter {
        print!("{}", val)
    }
}

fn demo_next() {
    println!("----------");
    let arr = [1, 2, 3, 4];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), Some(4));
    assert_eq!(arr_iter.next(), None);

    // zip，map，filter, skip has default impl
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

fn demo_impl_for() {
    let value = vec![1, 2, 3];
    let result = match IntoIterator::into_iter(value) {
        mut iter => loop {
            match iter.next() {
                Some(x) => {
                    println!("{}", x)
                }
                None => break,
            }
        },
    };
    result
}

/*
 into_iter 会夺走所有权
iter 是借用
iter_mut 是可变借用
 */

fn demo_3_inters() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}

/*
 .iter() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&T)
.iter_mut() 方法实现的迭代器，调用 next 方法返回的类型是 Some(&mut T)，
因此在 if let Some(v) = values_iter_mut.next() 中，
v 的类型是 &mut i32，最终我们可以通过 *v = 0 的方式修改其值
 */

fn demo_consumer_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);
}

fn demo_collect() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, [2, 3, 4]);

    use std::collections::HashMap;

    let names = ["james", "jack"];
    let ages = [7, 100];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn new(s: u32, sy: String) -> Shoe {
        Shoe { size: s, style: sy }
    }
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn demmo_filter() {
    let shoes: Vec<Shoe> = vec![
        Shoe::new(12, "Sports".to_string()),
        Shoe::new(16, "Boots".to_string()),
    ];
    // let selected = shoes_in_size(shoes, 16);
    // println!("{:?}", selected);

    let empty = shoes_in_size(shoes, 10002);
    println!("{:?}", empty);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn demo_implement_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn demo_others() {
    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("第{}个值是{}", i, v)
    }

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v
        .iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);

    println!("{}", val);
}
