pub fn demo_it() {
    demo_basic();
    demo_basic_2();
    demo_address_and_pointer();
    demo_try_into();
}

fn demo_basic() {
    let a = 10_i32;
    let b = 100_u16;

    if a < b as i32 {
        println!("a<b")
    }
}

fn demo_basic_2() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;

    println!("{}, {}, {}", a, b, c);
}

fn demo_address_and_pointer() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
    println!("{:?}", values);
}

fn demo_try_into() {
    // use std::convert::TryInto;
    let b: i16 = 1500;

    // let b_: u8 = b.try_into().unwrap(); //unwrap will panic
    let _b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => { 
            println!("{:?}", e.to_string());
            0
        }
    };
}
