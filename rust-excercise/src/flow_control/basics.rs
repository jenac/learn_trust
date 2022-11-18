pub fn demo_it() {
    basic_demo();
    if_else_demo();
    for_demo();
    continue_demo();
    break_demo();
    while_demo();
    break_with_return_value();
}

fn basic_demo() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn if_else_demo() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn for_demo() {
    for i in 1..=5 {
        println!("{}", i);
    }
    println!();
    for i in 11..15 {
        println!("{}", i);
    }
    println!();
    let a = [4,3,2,1];
    for (i, v) in a.iter().enumerate() {
        println!("No {} element is {}", i+1, v)

    }
}

//注意，使用 for 时我们往往使用集合的引用形式，
//除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 container 的引用）。
//如果不使用引用的话，所有权会被转移（move）到 for 语句块中，后面就无法再使用这个集合了)：
/*
for item in &container {
  // ...
}
*/
//如果想在循环中，修改该元素，可以使用 mut 关键字：
/*
使用方法	等价使用方式	所有权
for item in collection	for item in IntoIterator::into_iter(collection)	转移所有权
for item in &collection	for item in collection.iter()	不可变借用
for item in &mut collection	for item in collection.iter_mut()	可变借用
*/

fn continue_demo() {
    println!("continue_demo");
    for i in 1..4 {
        if i==2 {
            continue;
        }
        println!("{}", i)
    }
}

fn break_demo() {
    println!("break demo");
    for i in 1..4 {
        if i==2 {
            break;
        }
        println!("{}", i)
    }
}

fn while_demo() {
    println!("while demo");
    let mut n=0;
    while n<= 5 {
        println!("{}", n);
        n+=1;
    }
    println!("I am here!");
}

fn break_with_return_value() {
    let mut counter = 0;
    let result = loop{
        counter +=1;
        if counter == 20 {
            break counter + 2;
        }
    };
    println!("result = {}", result);
}