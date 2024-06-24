use ark_ec::Group;
// use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_std::iterable::Iterable;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::iter::{once, zip};

use crate::{
    encryption::encrypt,
    types::{EncryptedValue, EncryptedValueType, KeyType, KeyTypeType},
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ChallProof {
    challenge: KeyType,
    proof: KeyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShuffleWithProof {
    pub values_prev: Vec<EncryptedValue>,
    pub values_aftr: Vec<EncryptedValue>,
    pub public_key: EncryptedValue,
    pub proofs: Vec<Vec<ChallProof>>,
}

fn hash_to_field(hash: &[u8; 32]) -> KeyType {
    let mut h1 = 0u128;
    for d in hash.iter().take(16) {
        h1 = h1 * 256u128 + d as u128;
    }
    let mut h2 = 0u128;
    for d in hash.iter().skip(16) {
        h2 = h2 * 256u128 + d as u128;
    }

    let mul = KeyTypeType::from(u128::MAX) + KeyTypeType::from(1);
    let res = (KeyTypeType::from(h1) * mul) + KeyTypeType::from(h2);
    KeyType::new(res)
}

fn calc_hash(points: &Vec<EncryptedValue>) -> KeyType {
    let mut hasher = Sha256::new();
    for point in points {
        hasher.update(&point.to_string());
    }
    hash_to_field(&hasher.finalize().into())
}

fn linear_combination(
    a: &KeyType,
    x: &EncryptedValue,
    b: &KeyType,
    y: &EncryptedValue,
) -> EncryptedValue {
    EncryptedValue::new(encrypt(x, a).val + encrypt(y, b).val)
}

fn one_in_n<R>(
    p_key: &KeyType,
    g: &EncryptedValue,
    points: &[EncryptedValue],
    ind: usize,
    rng: &mut R,
) -> Vec<ChallProof>
where
    R: Rng,
{
    dbg!("one in n");
    dbg!(&ind);
    let n = points.len();
    let mut res = vec![None; n];
    let p = encrypt(points.get(ind).unwrap(), p_key);
    let r = KeyType::rand(rng);
    let public_key = encrypt(g, p_key);
    let r_i = encrypt(points.get(ind).unwrap(), &r);
    let r_y = encrypt(g, &r);
    let mut e = calc_hash(
        &points
            .iter()
            .copied()
            .chain(once(r_y).chain(once(p)).chain(once(r_i)))
            .collect(),
    );
    for i in (ind + 1..n).chain(0..ind) {
        let s = KeyType::rand(rng);
        let r_i = linear_combination(&e, &p, &s, points.get(i).unwrap());
        let r_y = linear_combination(&e, &public_key, &s, g);
        *res.get_mut(i).unwrap() = Some(ChallProof {
            challenge: e,
            proof: s,
        });
        e = calc_hash(
            &points
                .iter()
                .copied()
                .chain(once(r_y))
                .chain(once(p))
                .chain(once(r_i))
                .collect(),
        );
    }
    let s = KeyType::new(r.val - (e.val * p_key.val));
    *res.get_mut(ind).unwrap() = Some(ChallProof {
        challenge: e,
        proof: s,
    });
    assert!(res.iter().all(|f| f.is_some()));
    res.into_iter().flatten().collect()
}

fn verify_1_in_n(
    n: usize,
    p: &EncryptedValue,
    g: &EncryptedValue,
    points: &[EncryptedValue],
    public_key: &EncryptedValue,
    proof: &[ChallProof],
) -> bool {
    for ind in 0..n {
        let ChallProof {
            challenge: e,
            proof: s,
        } = proof.get(ind).unwrap();
        let r = linear_combination(e, p, s, points.get(ind).unwrap());
        let r_y = linear_combination(e, public_key, s, g);
        let e_next = calc_hash(
            &points
                .iter()
                .copied()
                .chain(once(r_y))
                .chain(once(*p))
                .chain(once(r))
                .collect(),
        );
        let e_next_real = &proof
            .get(if ind == n - 1 { 0 } else { ind + 1 })
            .unwrap()
            .challenge;
        if e_next != *e_next_real {
            return false;
        };
    }
    true
}

impl ShuffleWithProof {
    pub fn generate<R>(
        values_prev: Vec<EncryptedValue>,
        p_key: &KeyType,
        perm: &Vec<usize>,
        rng: &mut R,
    ) -> Self
    where
        R: Rng,
    {
        assert_eq!(perm.len(), values_prev.len());
        let g = EncryptedValue::new(EncryptedValueType::generator());
        let public_key = encrypt(&g, p_key);
        let values_aftr = perm
            .iter()
            .map(|&ind| encrypt(values_prev.get(ind).unwrap(), p_key))
            .collect();
        let proofs = perm
            .iter()
            .map(|&ind| one_in_n(p_key, &g, &values_prev, ind, rng))
            .collect();
        ShuffleWithProof {
            values_prev,
            values_aftr,
            public_key,
            proofs,
        }
    }

    pub fn verify(&self, prev: &Vec<EncryptedValue>) -> bool {
        if self.values_prev != *prev {
            return false;
        }
        let g = EncryptedValue::new(EncryptedValueType::generator());
        let n = self.values_prev.len();
        for i in 0..n {
            for j in i + 1..n {
                if self.values_aftr.get(i).unwrap() == self.values_aftr.get(j).unwrap() {
                    return false;
                }
            }
        }
        for (point, proof) in zip(self.values_aftr.iter(), self.proofs.iter()) {
            if !verify_1_in_n(n, point, &g, &self.values_prev, &self.public_key, proof) {
                return false;
            }
        }
        true
    }
}

fn mask_proof<R>(p: &EncryptedValue, k: &KeyType, pp: &EncryptedValue, rng: &mut R) -> ChallProof
where
    R: Rng,
{
    let r = KeyType::rand(rng);
    let r_p = encrypt(p, &r);
    let e = calc_hash(&vec![*p, *pp, r_p]);
    let s = KeyType::new(r.val - (e.val * k.val));
    ChallProof {
        challenge: e,
        proof: s,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptWithProof {
    pub values_prev: Vec<EncryptedValue>,
    pub values_aftr: Vec<EncryptedValue>,
    pub proofs: Vec<ChallProof>,
}

impl EncryptWithProof {
    pub fn generate<R>(values_prev: Vec<EncryptedValue>, keys: &Vec<KeyType>, rng: &mut R) -> Self
    where
        R: Rng,
    {
        let (proofs, values_aftr) = zip(values_prev.iter(), keys.iter())
            .map(|(p, k)| {
                let pp = encrypt(p, k);
                (mask_proof(p, k, &pp, rng), pp)
            })
            .unzip();
        EncryptWithProof {
            values_prev,
            values_aftr,
            proofs,
        }
    }
    pub fn verify(&self, prev: &Vec<EncryptedValue>) -> bool {
        if self.values_prev != *prev {
            return false;
        }
        for (p, (pp, proof)) in self
            .values_prev
            .iter()
            .zip(zip(self.values_aftr.iter(), self.proofs.iter()))
        {
            let ChallProof {
                challenge: e,
                proof: s,
            } = proof;
            let r_p = linear_combination(e, pp, s, p);
            let e_v = calc_hash(&vec![*p, *pp, r_p]);
            if *e != e_v {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use ark_std::test_rng;
    use rand::seq::SliceRandom;

    use crate::{
        encryption::{basic_deck, decrypt, short_deck, Translator},
        types::KeyType,
    };

    use super::{EncryptWithProof, ShuffleWithProof};

    #[test]
    fn shuffle_with_proof() {
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
        println!();
        dbg!("Start verification");
        assert!(proof.verify(&deck));
        dbg!("Done verification");
    }

    #[test]
    fn encrypt_with_proof() {
        let mut rng = test_rng();
        let deck = basic_deck().to_vec();
        let keys = vec![0; deck.len()]
            .iter()
            .map(|_| KeyType::rand(&mut rng))
            .collect();
        dbg!("Start generating proof");
        let proof = EncryptWithProof::generate(deck.clone(), &keys, &mut rng);
        dbg!("Done generating proof");
        let trans = Translator::new(&basic_deck());
        for (v, k) in proof.values_aftr.iter().zip(keys.iter()) {
            print!("{} ", trans.translate(decrypt(v, k)).unwrap());
        }
        println!();
        dbg!("Start verification");
        assert!(proof.verify(&deck));
        dbg!("Done verification");
    }

    #[test]
    fn shuffle_with_proof_short() {
        let mut rng = test_rng();
        let deck = short_deck().to_vec();
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
        println!();
        dbg!("Start verification");
        assert!(proof.verify(&deck));
        dbg!("Done verification");
    }

    #[test]
    fn encrypt_with_proof_short() {
        let mut rng = test_rng();
        let deck = short_deck().to_vec();
        let keys = vec![0; deck.len()]
            .iter()
            .map(|_| KeyType::rand(&mut rng))
            .collect();
        dbg!("Start generating proof");
        let proof = EncryptWithProof::generate(deck.clone(), &keys, &mut rng);
        dbg!("Done generating proof");
        let trans = Translator::new(&basic_deck());
        for (v, k) in proof.values_aftr.iter().zip(keys.iter()) {
            print!("{} ", trans.translate(decrypt(v, k)).unwrap());
        }
        println!();
        dbg!("Start verification");
        assert!(proof.verify(&deck));
        dbg!("Done verification");
    }
}
