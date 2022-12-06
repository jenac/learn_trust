#[allow(unused)]
pub fn demo_it() {
    demo_lt_same();
    demo_fn_takes_shortest();
    demo_lt_with_struct();
    demo_lt_elision();
}

fn demo_lt_same() {
    let string1 = String::from("abv");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The logest one is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo_fn_takes_shortest() {
    let string1 = String::from("abc");
    let result;
    {
        let string2 = "xyz";
        // the following fail, sicne string2 lifetime is within {}
        // let string2 = String::from("xyz");
        result = longest(string1.as_str(), &string2);
    }
    println!("The longest is {}", result);
}

#[derive(Debug)]
#[allow(unused)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn demo_lt_with_struct() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("{:?}", i);
    }
    //this will fail to compile at line 45: borrowed value does not live long enough
    // println!("{:?}",i);
}

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn annonce_and_return_part(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Announcement: {}", announcement);
        self.part
    }
}
fn demo_lt_elision() {
    let i = ImportantExcerpt { part: "hello" };
    i.annonce_and_return_part("announcement");
}
