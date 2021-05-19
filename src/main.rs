use std::io;

fn main() {
    e2_2();
    println!("----------");
    e2_1();
    
}

fn e2_1() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}

fn e2_2() {
    let x = 5;
    let y = 6;
    println!("x={}, y={}", x, y)
}
