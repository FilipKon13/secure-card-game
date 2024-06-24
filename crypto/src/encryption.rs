use ark_ec::Group;
use ark_ff::Field;
use ark_std::iterable::Iterable;

use rand::thread_rng;

use crate::types::{EncryptedValue, EncryptedValueType, KeyType};

pub struct Translator {
    cards: [EncryptedValue; 52],
}

pub fn basic_deck() -> [EncryptedValue; 52] {
    let g = EncryptedValueType::generator();
    let mut actual = -g;
    core::array::from_fn(|_| {
        actual += g;
        EncryptedValue::new(actual)
    })
}

pub fn short_deck() -> [EncryptedValue; 16] {
    let g = EncryptedValueType::generator();
    let mut res = [EncryptedValue::new(g); 16];
    for i in 0..4 {
        for j in 9..13 {
            let ind = i * 13 + j;
            *res.get_mut(i * 4 + j - 9).unwrap() =
                EncryptedValue::new(g * KeyType::from(ind as i64).val);
        }
    }
    res
}

impl Translator {
    pub fn new(deck: &[EncryptedValue; 52]) -> Self {
        Translator { cards: *deck }
    }
    pub fn translate(&self, value: EncryptedValue) -> Option<usize> {
        self.cards.iter().position(|v| v == value)
    }
}

pub fn encrypt(message: &EncryptedValue, p_key: &KeyType) -> EncryptedValue {
    EncryptedValue::new(message.val * p_key.val)
}

pub fn decrypt(message: &EncryptedValue, p_key: &KeyType) -> EncryptedValue {
    EncryptedValue::new(message.val * p_key.val.inverse().unwrap())
}

pub fn rand_key() -> KeyType {
    KeyType::rand(&mut thread_rng())
}

pub fn mul_key(a: &KeyType, b: &KeyType) -> KeyType {
    KeyType::new(a.val * b.val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::test_rng;
    use ark_std::UniformRand;

    #[test]
    fn starting_deck() {
        let deck = Translator::new(&basic_deck()).cards;
        let g = EncryptedValue::new(EncryptedValueType::generator());
        for (i, v) in deck.iter().enumerate() {
            assert_eq!(v, encrypt(&g, &KeyType::from(i as i64)));
        }
    }

    #[test]
    fn translation() {
        let translator = Translator::new(&basic_deck());
        const INDEX: i64 = 10;
        let elem = encrypt(
            &EncryptedValue::new(EncryptedValueType::generator()),
            &KeyType::from(INDEX),
        );
        assert_eq!(INDEX as usize, translator.translate(elem).unwrap());
    }

    #[test]
    #[should_panic]
    fn translation_fail() {
        let translator = Translator::new(&basic_deck());
        const INDEX: i64 = 60;
        let elem = encrypt(
            &EncryptedValue::new(EncryptedValueType::generator()),
            &KeyType::from(INDEX),
        );
        translator.translate(elem).unwrap();
    }

    #[test]
    fn test_encrypt_decrypt() {
        let mut rng = test_rng();
        let plaintext = EncryptedValue::new(EncryptedValueType::rand(&mut rng));
        let p_key = KeyType::rand(&mut rng);
        let ciphertext = encrypt(&plaintext, &p_key);
        let plaintext_p = decrypt(&ciphertext, &p_key);
        assert_eq!(plaintext, plaintext_p);
    }

    #[test]
    #[should_panic]
    fn test_encrypt_decrypt_fail() {
        let mut rng = test_rng();
        let plaintext = EncryptedValue::new(EncryptedValueType::rand(&mut rng));
        let p_key = KeyType::rand(&mut rng);
        let rand_key = KeyType::rand(&mut rng);
        let ciphertext = encrypt(&plaintext, &p_key);
        let plaintext_p = decrypt(&ciphertext, &rand_key);
        assert_eq!(plaintext, plaintext_p);
    }
}
