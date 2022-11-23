#[allow(unused)]

pub fn demo_it() {
    circle_demo();
    rectangle_demo();
    impl_enum();
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn circle_demo() {
    let c = Circle::new(12.0, 12.0, 6.0);
    println!(
        "circle: x={},y={},r={}.  area = {}",
        c.x,
        c.y,
        c.radius,
        c.area()
    );
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //在 Rust 中，允许方法名跟结构体的字段名相同：
    fn width(&self) -> bool {
        self.width > 0
    }

    //as getter
    //this also makes height private
    fn height(&self) -> u32 {
        self.height
    }
}

// can have multiple impl
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn rectangle_demo() {
    let r = Rectangle{width: 12, height:18};
    println!("rectangle={:?}, area = {}", r, r.area());
    println!("width > 0? {}", r.width());
    println!("use getter: {}", r.height());

    println!("----------");
    let r1 = Rectangle{width:30, height:50};
    let r2 = Rectangle{width:10, height:40};
    let r3 = Rectangle{width:60, height:45};

    println!("can r1 hold r2? {}", r1.can_hold(&r2));
    println!("can r1 hold r3? {}", r1.can_hold(&r3));
}

/*需要注意的是，self 依然有所有权的概念：

self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
&self 表示该方法对 Rectangle 的不可变借用
&mut self 表示可变借用

回到上面的例子中，选择 &self 的理由跟在函数中使用 &Rectangle 是相同的：
我们并不想获取所有权，也无需去改变它，只是希望能够读取结构体中的数据。
如果想要在方法中去改变当前的结构体，需要将第一个参数改为 &mut self。
仅仅通过使用 self 作为第一个参数来使方法获取实例的所有权是很少见的，
这种使用方式往往用于把当前的对象转成另外一个对象时使用，转换完后，
就不再关注之前的对象，且可以防止对之前对象的误调用。
*/

//关联函数 static method in C# ?

// impl enum
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn as_string(&self) -> String {
        match self {
            Self::Quit => "Quit".to_string(),
            Self::Move{x: a, y: b} => {
                format!("Move {}, {}", a, b)
            },
            Self::Write(s) => s.clone(),
            Self::ChangeColor(r, g, b) => {
                format!("Change rgb = {},{},{}", r, g, b)
            }
        }
    }
}

fn impl_enum() {
    let m = Message::ChangeColor(16, 32, 64);
    println!("message={}", m.as_string());
}
