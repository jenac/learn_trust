use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
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
