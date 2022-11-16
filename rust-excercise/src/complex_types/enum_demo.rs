#![allow(unused)]
pub fn demo_it() {
    use_enum_demo();
    use_in_struct();
    use_better_poker_card();
    use_mixed_poker_card();
    complex_enum_demo();
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn use_enum_demo() {
    let heart = PokerSuit::Hearts;
    let diamod = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamod);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

fn use_in_struct() {
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };

    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
  println!("{:?}, {:?}", c1, c2);
}

#[derive(Debug)]
enum BetterPokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn use_better_poker_card() {
    let c1 = BetterPokerCard::Spades(6);
    let c2 = BetterPokerCard::Diamonds(13);

    println!("{:?}, {:?}", c1, c2)
}

#[derive(Debug)]
enum MixedPokerCard {
    C(u8),
    S(u8),
    D(char),
    H(char),
}

fn use_mixed_poker_card() {
    let c1 = MixedPokerCard::S(6);
    let c2 = MixedPokerCard::D('K');

    println!("{:?}, {:?}", c1, c2)
}

//从这些例子可以看出，任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举。
#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 该枚举类型代表一条消息，它包含四个不同的成员：

// Quit 没有任何关联数据
// Move 包含一个匿名结构体
// Write 包含一个 String 字符串
// ChangeColor 包含三个 i32
// 当然，我们也可以用结构体的方式来定义这些消息：
/*
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
*/

fn complex_enum_demo() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:5, y:8};
    let m3 = Message::ChangeColor(244, 211, 255);
    println!("{:?}, {:?}, {:?}", m1, m2, m3);

}