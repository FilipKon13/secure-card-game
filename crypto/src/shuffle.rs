use crate::encryption::{decrypt, encrypt, rand_key};
use crate::types::{EncryptedValue, KeyType};
use rand::{seq::SliceRandom, thread_rng};

/*
 * Shuffling proceeds as follows:
 * Assume that deck is {d_0, ..., d_51}
 * A encrypts with a and shuffles {ad_0, ..., ad_51}
 * B encrypts with b and shuffles what was received from A
 * C encrypts with c and shuffles what was received from B
 * Permutation is now determined
 * A decrypts with a and encrypts i-th card with a_i
 * B decrypts with b and encrypts i-th card with b_i
 * C decrypts with c and encrypts i-th card with c_i
 * Everyone has the same version now
 *
 * This shuffling method works with trusted parties, i.e. if nobody tries to cheat,
 * then the deck is uniformly shuffled and nobody knows anything about the permutation.
 *
 * To ensure that nobody cheated during shuffling
 * (e.g. it is easy to try to insert copy of the same card into the deck)
 * at the end of the game everybody reveals their keys and everyone can
 * verify that deck is restored. Otherwise, game has to be invalidated.
 */

#[derive(Debug, PartialEq, Eq)]
pub enum PartyState {
    WaitForShuffle,
    WaitForEncryption,
    WaitForDeck,
    Done,
}
pub struct PartyBasic {
    pub state: PartyState,
    keys: Vec<KeyType>,
    deck: Vec<EncryptedValue>,
}

impl PartyBasic {
    pub fn new() -> Self {
        PartyBasic {
            state: PartyState::WaitForShuffle,
            keys: vec![],
            deck: vec![],
        }
    }
    fn encrypt_and_shuffle(&mut self, deck: &mut Vec<EncryptedValue>) {
        assert_eq!(self.state, PartyState::WaitForShuffle, "Illegal state");
        let key = rand_key();
        self.keys.push(key);
        for card in deck.iter_mut() {
            *card = encrypt(*card, key);
        }
        deck.shuffle(&mut thread_rng());
        self.state = PartyState::WaitForEncryption;
    }
    fn decrypt_encrypt(&mut self, deck: &mut Vec<EncryptedValue>) {
        assert_eq!(self.state, PartyState::WaitForEncryption, "Illegal state");
        let key = *self.keys.first().unwrap();
        self.keys.pop();
        for card in deck.iter_mut() {
            let new_key = rand_key();
            self.keys.push(new_key);
            *card = encrypt(decrypt(*card, key), new_key);
        }
        self.state = PartyState::WaitForDeck;
    }
    fn pass_deck(&mut self, deck: &Vec<EncryptedValue>) {
        match self.state {
            PartyState::WaitForDeck => self.deck = deck.clone(),
            PartyState::Done => return,
            _ => panic!("Illegal state"),
        };
        self.state = PartyState::Done;
    }
    pub fn make_turn(&mut self, deck: &mut Vec<EncryptedValue>) {
        match self.state {
            PartyState::WaitForShuffle => self.encrypt_and_shuffle(deck),
            PartyState::WaitForEncryption => self.decrypt_encrypt(deck),
            PartyState::WaitForDeck => self.pass_deck(deck),
            PartyState::Done => return,
        };
    }
    pub fn is_done(&self) -> bool {
        return self.state == PartyState::Done;
    }
    pub fn retrieve_deck(self) -> (Vec<EncryptedValue>, Vec<KeyType>) {
        assert_eq!(self.state, PartyState::Done);
        (self.deck, self.keys)
    }
}

#[cfg(test)]
mod test {
    use crate::encryption::{basic_deck, Translator};

    use super::*;

    fn prepare_players() -> Vec<PartyBasic> {
        vec![PartyBasic::new(), PartyBasic::new(), PartyBasic::new()]
    }

    #[test]
    fn validate_preparation() {
        let players = prepare_players();
        for player in players.iter() {
            assert_eq!(player.state, PartyState::WaitForShuffle);
        }
    }

    fn run_protocol(players: &mut Vec<PartyBasic>) {
        let mut deck = basic_deck().into_iter().collect();
        loop {
            let mut done = true;
            for player in players.iter_mut() {
                player.make_turn(&mut deck);
                done &= player.state == PartyState::Done;
            }
            if done {
                break;
            }
        }
    }

    #[test]
    fn check_decks() {
        let mut players = prepare_players();
        run_protocol(&mut players);
        let (decks, keys): (Vec<_>, Vec<_>) =
            players.into_iter().map(|p| p.retrieve_deck()).unzip();
        assert!(decks.windows(2).all(|w| w[0] == w[1]));
        assert!(decks.iter().all(|d| d.len() == 52));
        assert!(keys.iter().all(|k| k.len() == 52));
    }

    #[test]
    fn decryption() {
        use std::iter::zip;
        let mut players = prepare_players();
        run_protocol(&mut players);
        let mut deck = players.first().unwrap().deck.clone();
        for player in players.iter() {
            for (value, &key) in zip(deck.iter_mut(), &player.keys) {
                *value = decrypt(*value, key);
            }
        }
        let translator = Translator::new(&basic_deck());
        let mut perm = deck
            .into_iter()
            .map(|v| translator.translate(v))
            .flatten()
            .collect::<Vec<usize>>();
        assert_ne!(perm, { 0..52 }.collect::<Vec<usize>>());
        perm.sort();
        assert_eq!(perm, { 0..52 }.collect::<Vec<usize>>());
    }
}
