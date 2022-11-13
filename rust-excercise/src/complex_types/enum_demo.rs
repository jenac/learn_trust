#![allow(unused)]
pub fn demo_it() {
    use_enum_demo();
  use_in_struct();
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
