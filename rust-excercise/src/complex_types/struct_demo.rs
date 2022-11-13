#![allow(unused)]
pub fn demo_it() {
    struct_intro();
    build_user_demo();
    struct_in_mem();
    tuple_struct();
    debug_print();
    debug_2();
    ownership();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn struct_intro() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone1"),
        active: true,
        sign_in_count: 12,
    };

    println!("{:?}", user1.email);
    // user1.email = String::from("james@apple.com"); // need mut user1 to change
    println!("{:?}", user1.email);
}

fn build_user_demo() {
    let u = build_user(String::from("a@sina.com"), String::from("a_user"));
    println!("{:?}", u.email);

    //haha!
    let u2 = User {
        email: String::from("just_diff@sian.com"),
        ..u //just 2 dots
    };
    println!("u2: email={}, user={}", u2.email, u2.username);
    println!("u: email={}, user={}", u.email, "u.username mvoed"); //u.username moved out, but email not
}

fn build_user(email: String, username: String) -> User {
    User {
        email, //same as ts.
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn struct_in_mem() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

fn tuple_struct() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(1, 2, 3);
    let origin = Point(4, 5, 6);

    println!("{}", black.0);
    println!("{}", origin.2);

    println!("{:?}", black);
    println!("{:?}", origin);
}

fn unit_like_struct() {
    // struct AlwaysEqual;

    // let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {}
}

// 在前面的代码中我们使用 #[derive(Debug)] 对结构体进行了标记，
// 这样才能使用 println!("{:?}", s); 的方式对其进行打印输出，
//当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 {:#?} 来替代 {:?}，
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn debug_print() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
}

fn debug_2() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn ownership() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}
