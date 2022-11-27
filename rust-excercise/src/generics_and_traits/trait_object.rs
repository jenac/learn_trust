#[allow(unused)]
pub fn demo_it() {
    demo_run_screen();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button={},{},{}", self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectButton @ {}, {} with {} options",
            self.width,
            self.height,
            self.options.len()
        );
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn demo_run_screen() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 32,
                options: vec!["Yes".to_string(), "No".to_string(), "Later".to_string()],
            }),
            Box::new(Button {
                width: 100,
                height: 60,
                label: String::from("USER"),
            }),
        ],
    };

    screen.run();

    println!("----- array of trait objects -----");
    let component_array: [Box<dyn Draw>; 2] = [Box::new(SelectBox {
        width: 75,
        height: 32,
        options: vec!["Yes".to_string(), "No".to_string(), "Later".to_string()],
    }),
    Box::new(Button {
        width: 100,
        height: 60,
        label: String::from("USER"),
    }),];

    for component in component_array {
        component.draw();
    }
    // the following does't compile
    // let s = Screen{
    //     components: vec![Box::new(String::from("IK")),]
    // }
}

