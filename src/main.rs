use common::add;
use common::cards::*;

fn main() {
    let card = PlayingCard::Up(Card {
        suit: Suit::Diamonds,
        rank: Rank::Ace,
    });
    let card_not_seen = PlayingCard::Hidden;
    println!("Hello, world!");
    println!("My cards: {:?}, {:?}", card, card_not_seen);
    println!("Test add: {}", add(5, 4));
}
