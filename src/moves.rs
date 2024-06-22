use common::cards::Card;
use crypto::{encryption::decrypt, types::KeyType};
use network::connection::Connection;

use crate::player::{Owner, Player};

pub trait Moves {
    fn draw_from_deck(&mut self) -> Card;
    fn let_draw_from_deck(&mut self, other: usize);
    fn play_card(&mut self, ind: usize);
    fn let_play_card(&mut self, other: usize) -> Card;
}

fn get_top_of_deck(player: &mut Player) -> usize {
    player
        .owners
        .iter()
        .position(|o| return o.is_none())
        .unwrap()
}

impl Moves for Player {
    fn draw_from_deck(&mut self) -> Card {
        let ind = get_top_of_deck(self);
        let mut encrypted_card = *self.deck.get(ind).unwrap();
        self.players
            .iter_mut()
            .map(|o| -> KeyType {
                o.send(&ind);
                o.receive()
            })
            .for_each(|k| encrypted_card = decrypt(encrypted_card, k));
        let decrypted_card = decrypt(encrypted_card, *self.keys.get(ind).unwrap());
        let card = self
            .translator
            .translate(decrypted_card)
            .expect("Other player did not provide right key");

        *self.owners.get_mut(ind).unwrap() = Some(Owner::Me);
        Card::try_from(card).unwrap()
    }

    fn let_draw_from_deck(&mut self, other: usize) {
        let ind = get_top_of_deck(self);
        let player = self.players.get_mut(other).unwrap();
        assert_eq!(
            ind,
            player.receive::<usize>(),
            "Other player asks for wrong card"
        );
        *self.owners.get_mut(ind).unwrap() = Some(Owner::Other(other));
        player.send(self.keys.get(ind).unwrap());
    }

    // works only for two player game atm
    fn play_card(&mut self, ind: usize) {
        self.players
            .iter_mut()
            .for_each(|o| o.send(&(ind, self.keys.get(ind).unwrap())));
    }

    // works only for two player game atm
    fn let_play_card(&mut self, other: usize) -> Card {
        let player = self.players.get_mut(other).unwrap();
        let (ind, key) = player.receive::<(usize, KeyType)>();
        match self
            .owners
            .get(ind)
            .expect("Wrong index")
            .expect("Player is not owner of this card")
        {
            Owner::Me => panic!("Player is not owner of this card"),
            Owner::Other(owner) => assert_eq!(owner, other, "Player is not owner of this card"),
        };
        let decrypted_card = decrypt(
            decrypt(*self.deck.get(ind).unwrap(), key),
            *self.keys.get(ind).unwrap(),
        );
        let card = self
            .translator
            .translate(decrypted_card)
            .expect("Other player did not provide right key");
        Card::try_from(card).unwrap()
    }
}
