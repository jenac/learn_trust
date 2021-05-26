use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    e3_4();
    println!("----------");
    e3_3();
    println!("----------");
    e3_2();
    println!("----------");
    e3_1();
    println!("----------");
    e2_2();
    println!("----------");
    e2_1();
    
}

fn e2_1() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("need a number"); continue },
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less =>  println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; }
        }
    }
}

fn e2_2() {
    let x = 5;
    let y = 6;
    println!("x={}, y={}", x, y)
}

fn e3_1() {
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x now = {}", x);
}

fn e3_2() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
}

fn e3_3() {
    let x = 5; // immutable x
    let x = x+1; //new x shadow old x
    let x = x*2; //new x shadow x again
    println!("x is now = {}", x);
}

fn e3_4() {
    let spaces = "   ";
    let spaces = spaces.len(); //good, new variable with different type
    println!("spaces = {}", spaces);

    let mut spaces = " SS  "; //ok, shadow again, make it mutable
    spaces = "GOOG";
    // spaces = spaces.len(); //error, no shadow, variable type cannot change
    println!("spaces = {}", spaces);
}