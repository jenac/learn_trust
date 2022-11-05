struct Struct {
    e: i32
}

fn main() {
    let mut x = 5;
    println!("x={}", x);
    x = 6;
    println!("x={}", x);

    //使用下划线开头忽略未使用的变量
    let _x = 5;

    //变量解构
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    //解构式赋值
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    /*
    常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
    */
    const MAX_POINTS: u32 = 100_000;

    /*
    Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    */
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    /*
    变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），
    就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。
    */
}


