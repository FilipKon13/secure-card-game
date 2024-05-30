use common::{
    cards::PlayingCard,
    game::{GameState, Player},
};

pub fn print_game<PlayerType>(game_state: GameState<PlayerType>)
where
    PlayerType: Player,
{
    let GameState {
        main_player,
        other_players,
        table_cards,
    } = game_state;
    println!("Game State:");
    println!("Table: {}", format_cards(table_cards.into_iter()));
    println!("Hand: {}", format_cards(main_player.cards().into_iter()));
    for (ind, player) in other_players.iter().enumerate() {
        println!(
            "Player {}: {}",
            ind,
            format_cards(player.cards().into_iter())
        );
    }
}

fn format_cards(cards: impl Iterator<Item = PlayingCard>) -> String {
    cards
        .map(|f| format!("{:?}", f))
        .collect::<Vec<String>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use common::{
        cards::*,
        game::{GameState, PlayerBasic},
    };

    use crate::print_game;

    #[test]
    fn it_works() {
        let hidden_player = PlayerBasic {
            cards: vec![
                PlayingCard::Hidden,
                PlayingCard::Hidden,
                PlayingCard::Hidden,
            ],
        };
        let game = GameState {
            main_player: PlayerBasic {
                cards: vec![
                    PlayingCard::Up(Card {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    }),
                    PlayingCard::Up(Card {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    }),
                    PlayingCard::Up(Card {
                        suit: Suit::Hearths,
                        rank: Rank::King,
                    }),
                ],
            },
            other_players: vec![hidden_player.clone(), hidden_player.clone()],
            table_cards: vec![
                PlayingCard::Hidden,
                PlayingCard::Hidden,
                PlayingCard::Up(Card {
                    suit: Suit::Clubs,
                    rank: Rank::Six,
                }),
            ],
        };
        print_game(game);
    }
}
