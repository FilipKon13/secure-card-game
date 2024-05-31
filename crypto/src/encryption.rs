use ark_ec::{short_weierstrass::Projective, Group};
use ark_ff::Field;
use ark_pallas::Fr as ScalarField;
use ark_pallas::{PallasConfig, Projective as G};
use ark_std::iterable::Iterable;
use ark_std::UniformRand;
use rand::thread_rng;

pub type EncryptedValue = Projective<PallasConfig>;
pub type KeyType = ScalarField;

pub struct Translator {
    cards: [EncryptedValue; 52],
}

pub fn basic_deck() -> [EncryptedValue; 52] {
    let g = G::generator();
    let mut actual = -g;
    core::array::from_fn(|_| {
        actual += g;
        actual
    })
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            cards: basic_deck(),
        }
    }
    pub fn translate(&self, value: EncryptedValue) -> Option<usize> {
        self.cards.iter().position(|v| v == value)
    }
}

pub fn encrypt(message: EncryptedValue, p_key: KeyType) -> EncryptedValue {
    message * p_key
}

pub fn decrypt(message: EncryptedValue, p_key: KeyType) -> EncryptedValue {
    message * p_key.inverse().unwrap()
}

pub fn rand_key() -> KeyType {
    ScalarField::rand(&mut thread_rng())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::test_rng;
    use ark_std::{ops::Mul, Zero};

    #[test]
    fn it_works() {
        let mut rng = test_rng();
        let a = G::rand(&mut rng);
        let b = G::rand(&mut rng);

        let c = a + b;
        let d = a - b;
        assert_eq!(c + d, a.double());
        let e = -a;
        assert_eq!(e + a, G::zero());

        let scalar = ScalarField::rand(&mut rng);
        let e = c.mul(scalar);
        let f = e.mul(scalar.inverse().unwrap());
        assert_eq!(f, c);
    }

    #[test]
    fn starting_deck() {
        let deck = Translator::new().cards;
        let g = G::generator();
        for (i, v) in deck.iter().enumerate() {
            assert_eq!(v, g.mul(ScalarField::from(i as u64)));
        }
    }

    #[test]
    fn translation() {
        let translator = Translator::new();
        const INDEX: u64 = 10;
        let elem = G::generator() * ScalarField::from(INDEX);
        assert_eq!(INDEX as usize, translator.translate(elem).unwrap());
    }

    #[test]
    #[should_panic]
    fn translation_fail() {
        let translator = Translator::new();
        const INDEX: u64 = 60;
        let elem = G::generator() * ScalarField::from(INDEX);
        translator.translate(elem).unwrap();
    }

    #[test]
    fn test_encrypt_decrypt() {
        let mut rng = test_rng();
        let plaintext = G::rand(&mut rng);
        let p_key = ScalarField::rand(&mut rng);
        let ciphertext = encrypt(plaintext, p_key);
        let plaintext_p = decrypt(ciphertext, p_key);
        assert_eq!(plaintext, plaintext_p);
    }

    #[test]
    #[should_panic]
    fn test_encrypt_decrypt_fail() {
        let mut rng = test_rng();
        let plaintext = G::rand(&mut rng);
        let p_key = ScalarField::rand(&mut rng);
        let rand_key = ScalarField::rand(&mut rng);
        let ciphertext = encrypt(plaintext, p_key);
        let plaintext_p = decrypt(ciphertext, rand_key);
        assert_eq!(plaintext, plaintext_p);
    }
}
