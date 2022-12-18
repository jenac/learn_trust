#[allow(unused)]
pub fn demo_it() {
    demo_conversion();
    demo_try_from();
    demo_transmute();
}

#[allow(dead_code)]
enum MyEnum {
    A = 1,
    B,
    C,
}

fn demo_conversion() {
    // 将枚举转换成整数，顺利通过
    let x = MyEnum::C as i32;
    println!("{}", x);

    // 将整数转换为枚举，失败
    // match x {
    //     MyEnum::A => {}
    //     MyEnum::B => {}
    //     MyEnum::C => {}
    //     _ => {}
    // }
}


//3rd party lib num_enums, nums-traits, num-derive

//TryFrom + 宏
use std::convert::TryFrom;

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

use std::convert::TryInto;
fn demo_try_from() {
    let x = MyEnum::C as i32;
    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}

//邪恶之王 std::mem::transmute
/*
最好使用#[repr(..)]来控制底层类型的大小，免得本来需要 i32，结果传入 i64，最终内存无法对齐，产生奇怪的结果
*/
#[repr(i32)]
#[allow(dead_code)]
enum MyEnum2 {
    A=1, B, C,
}

fn demo_transmute() {
    let x = MyEnum2::C;
    let y = x as i32;
    let z: MyEnum2 = unsafe{ std::mem::transmute(y)};

    match z {
        MyEnum2::A => println!("Found A"),
        MyEnum2::B => println!("Found B"),
        MyEnum2::C => println!("Found C"),
    }
}