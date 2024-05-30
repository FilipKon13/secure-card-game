use ark_ec::{short_weierstrass::Projective, Group};
use ark_pallas::{PallasConfig, Projective as G};
use ark_std::{iterable::Iterable, Zero};

pub type EncryptedValue = Projective<PallasConfig>;

pub struct Translator {
    cards: [EncryptedValue; 52],
}

impl Translator {
    pub fn new() -> Self {
        let g = G::generator();
        let mut actual = G::zero();
        let res = Translator {
            cards: core::array::from_fn(|_| {
                let old = actual;
                actual += g;
                old
            }),
        };
        res
    }
    pub fn translate(&self, value: EncryptedValue) -> Option<usize> {
        self.cards.iter().position(|v| v == value)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use ark_ff::Field;
    use ark_pallas::Fr as ScalarField;
    use ark_std::{ops::Mul, UniformRand};

    #[test]
    fn it_works() {
        let mut rng = ark_std::test_rng();
        // Let's sample uniformly random group elements:
        let a = G::rand(&mut rng);
        let b = G::rand(&mut rng);

        // We can add elements, ...
        let c = a + b;
        // ... subtract them, ...
        let d = a - b;
        // ... and double them.
        assert_eq!(c + d, a.double());
        // We can also negate elements, ...
        let e = -a;
        // ... and check that negation satisfies the basic group law
        assert_eq!(e + a, G::zero());

        // We can also multiply group elements by elements of the corresponding scalar field
        // (an act known as *scalar multiplication*)
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
        let elem = G::generator().mul(ScalarField::from(INDEX));
        assert_eq!(INDEX as usize, translator.translate(elem).unwrap());
    }

    #[test]
    #[should_panic]
    fn translation_fail() {
        let translator = Translator::new();
        const INDEX: u64 = 60;
        let elem = G::generator().mul(ScalarField::from(INDEX));
        translator.translate(elem).unwrap();
    }
}
