use crate::cards::PlayingCard;
pub trait Player {
    fn cards(&self) -> Vec<PlayingCard>;
}

pub struct GameState<PlayerType>
where
    PlayerType: Player,
{
    pub main_player: PlayerType,
    pub other_players: Vec<PlayerType>,
    pub table_cards: Vec<PlayingCard>,
}

#[derive(Debug, Clone)]
pub struct PlayerBasic {
    pub cards: Vec<PlayingCard>,
}

impl Player for PlayerBasic {
    fn cards(&self) -> Vec<PlayingCard> {
        self.cards.clone()
    }
}
