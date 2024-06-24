use ark_std::test_rng;
use crypto_scg::shuffle_v2::ShuffleWithProof;
use rand::seq::SliceRandom;

use crypto_scg::{
    encryption::{basic_deck, decrypt, Translator},
    types::KeyType,
};

fn main() {
    let mut rng = test_rng();
    let deck = basic_deck().to_vec();
    let p_key = KeyType::rand(&mut rng);
    let mut perm = vec![0usize; deck.len()];
    for i in 0..deck.len() {
        *perm.get_mut(i).unwrap() = i;
    }
    perm.shuffle(&mut rng);
    dbg!(&perm);
    dbg!("Start generating proof");
    let proof = ShuffleWithProof::generate(deck.clone(), &p_key, &perm, &mut rng);
    dbg!("Done generating proof");
    let trans = Translator::new(&basic_deck());
    for i in proof.values_aftr.iter() {
        print!("{} ", trans.translate(decrypt(i, &p_key)).unwrap());
    }
    dbg!();
    dbg!("Start verification");
    assert!(proof.verify(&deck));
    dbg!("Done verification");
}
