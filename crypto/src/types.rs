use std::fmt::Display;

use ark_ec::short_weierstrass::Projective;
use ark_ff::BigInt;
use ark_pallas::Fr as ScalarField;
use ark_pallas::PallasConfig;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::UniformRand;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub(crate) type EncryptedValueType = Projective<PallasConfig>;
pub(crate) type KeyTypeType = ScalarField;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct EncryptedValue {
    pub(crate) val: EncryptedValueType,
}

impl EncryptedValue {
    pub fn new(val: EncryptedValueType) -> Self {
        EncryptedValue { val }
    }
}

impl Display for EncryptedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(&self).unwrap().fmt(f)
    }
}

impl Serialize for EncryptedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = Vec::new();
        self.val.serialize_compressed(&mut bytes).unwrap(); // TODO convert this error to serde type (somehow)
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EncryptedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let val = Projective::deserialize_compressed(&*bytes).unwrap(); // TODO convert this error to serde type (somehow)
        Ok(EncryptedValue::new(val))
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct KeyType {
    pub(crate) val: ScalarField,
}

impl KeyType {
    pub fn new(val: ScalarField) -> Self {
        KeyType { val }
    }
    pub fn rand<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        KeyType::new(ScalarField::rand(rng))
    }
}

impl Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(&self).unwrap().fmt(f)
    }
}

impl<T> From<T> for KeyType
where
    ScalarField: From<T>,
{
    fn from(value: T) -> Self {
        KeyType::new(ScalarField::from(value))
    }
}

impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = Vec::new();
        self.val.serialize_compressed(&mut bytes).unwrap(); // TODO convert this error to serde type (somehow)
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let val = BigInt::<4>::deserialize_compressed(&*bytes).unwrap(); // TODO convert this error to serde type (somehow)
        Ok(KeyType::new(ScalarField::from(val)))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PartyState {
    WaitForShuffle,
    WaitForEncryption,
    WaitForDeck,
    Done,
}

#[cfg(test)]
mod test {
    use ark_std::test_rng;

    use crate::encryption::encrypt;

    use super::KeyType;
    use super::ScalarField;
    use super::{EncryptedValue, EncryptedValueType};
    use ark_ec::Group;
    use ark_std::UniformRand;
    use ark_std::Zero;

    #[test]
    fn test_eq() {
        let mut rng = test_rng();
        let g = EncryptedValue::new(EncryptedValueType::rand(&mut rng));
        let s1 = KeyType::rand(&mut rng);
        let s2 = KeyType::rand(&mut rng);
        let s = KeyType::new(s1.val * s2.val);
        let mut g1 = encrypt(&g, &s1);
        let mut g2 = encrypt(&g, &s2);
        g1 = encrypt(&g1, &s2);
        g2 = encrypt(&g2, &s1);
        let g3 = encrypt(&g, &s);
        let ser1 = serde_json::to_string(&g1).unwrap();
        let ser2 = serde_json::to_string(&g2).unwrap();
        let ser3 = serde_json::to_string(&g3).unwrap();
        assert_eq!(ser1, ser2);
        assert_eq!(ser1, ser3);
    }

    #[test]
    fn group_serialize_deserialize_gen() {
        let g = EncryptedValue::new(EncryptedValueType::generator());
        let serialized = serde_json::to_string(&g).unwrap();
        let deserialized: EncryptedValue = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, g);
    }

    #[test]
    fn group_serialize_deserialize_zero() {
        let g = EncryptedValue::new(EncryptedValueType::zero());
        let serialized = serde_json::to_string(&g).unwrap();
        let deserialized: EncryptedValue = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, g);
    }

    #[test]
    fn group_serialize_deserialize_rand() {
        let mut rng = test_rng();
        let g = EncryptedValue::new(EncryptedValueType::rand(&mut rng));
        let serialized = serde_json::to_string(&g).unwrap();
        let deserialized: EncryptedValue = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, g);
    }

    #[test]
    fn scalar_serialize_deserialize_zero() {
        let g = KeyType::from(0);
        let serialized = serde_json::to_string(&g).unwrap();
        let deserialized: KeyType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, g);
    }

    #[test]
    fn scalar_serialize_deserialize_gen() {
        let g = KeyType::from(1);
        let serialized = serde_json::to_string(&g).unwrap();
        let deserialized: KeyType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, g);
    }

    #[test]
    fn scalar_serialize_deserialize_rand() {
        let mut rng = test_rng();
        let g = KeyType::new(ScalarField::rand(&mut rng));
        let serialized = serde_json::to_string(&g).unwrap();
        let deserialized: KeyType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, g);
    }
}
