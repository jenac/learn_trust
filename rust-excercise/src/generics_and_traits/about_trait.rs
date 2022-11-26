#[allow(unused)]

pub fn demo_it() {
    demo_trait();
    demo_return_trait_dyn_impl();
}

pub trait Summary {
    fn summzrize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summzrize(&self) -> String {
        format!("title={}, author={}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summzrize(&self) -> String {
        format!("{} published {}", self.username, self.content)
    }
}

fn demo_trait() {
    let p = Post {
        title: "Rust Intro".to_string(),
        author: "James Bond".to_string(),
        content: "TLDR".to_string(),
    };

    let w = Weibo {
        username: "hk112".to_string(),
        content: "twwwwwweeeet!".to_string(),
    };

    println!("{}", p.summzrize());
    println!("{}", w.summzrize());

    println!("----- default summary:");
    println!("{}", p.default_summzrize());
    println!("{}", w.default_summzrize());

    println!("----- default summary call non-implelmented method");
    println!("{}", w.default_summzrize_partial());

    println!("----- passing in trait");
    passing_in_trait(&w);
    passing_in_trait(&p);

    println!("----- trait bound");
    trait_bound_demo(&p);
    trait_bound_demo(&w);

    println!("----- fixed largest");
    fixed_largest_demo();
}

pub trait DefaultSummary {
    fn default_summzrize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl DefaultSummary for Post {}

impl DefaultSummary for Weibo {
    fn default_summzrize(&self) -> String {
        format!(
            "I do not want default: {} published {}",
            self.username, self.content
        )
    }
}

//默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现
pub trait DefaultSummaryPartial {
    fn summzrize_author(&self) -> String;

    fn default_summzrize_partial(&self) -> String {
        format!(
            "I use summarize_author, you need implement it: {}",
            self.summzrize_author()
        )
    }
}

impl DefaultSummaryPartial for Weibo {
    fn summzrize_author(&self) -> String {
        format!("{}", self.username)
    }
}

fn passing_in_trait(item: &impl Summary) {
    println!("Breaking news! {}", item.summzrize());
}

fn trait_bound_demo<T: Summary>(item: &T) {
    println!("Trait bound: {}", item.summzrize());
}

//多重约束
//pub fn notify(item: &(impl Summary + Display)) {}
//pub fn notify<T: Summary + Display>(item: &T) {}

//Where 约束
/*
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
*/

//使用特征约束有条件地实现方法或特征
/*
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
cmp_display 方法，并不是所有的 Pair<T> 结构体对象都可以拥有，
只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法。
*/

//函数返回中的 impl Trait
#[allow(unused)]
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("hikao"),
        content: String::from("QWhat is it?"),
    }
}

//return trait dyn type
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "Sheeeeeep!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "Cooooooow!".to_string()
    }
}

fn randon_animal(randon_number: f64) -> Box<dyn Animal> {
    if randon_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn demo_return_trait_dyn_impl() {
    let random_number = 0.789;
    let animal = randon_animal(random_number);
    println!("random animal: {}", animal.noise());
}
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut res = list[0];

    for &item in list.iter() {
        if item > res {
            res = item;
        }
    }
    res
}

fn fixed_largest_demo() {
    let nums = vec![34, 23, 100, 12, 22, 89];

    let result = largest(&nums);
    println!("largest num={}", result);

    let chars = vec!['a', 'c', '1', 'z'];
    let result = largest(&chars);
    println!("largest char={}", result);
}
