use ark_ec::short_weierstrass::Projective;
use ark_ff::BigInt;
use ark_pallas::Fr as ScalarField;
use ark_pallas::PallasConfig;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::UniformRand;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub(crate) type EncryptedValueType = Projective<PallasConfig>;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct EncryptedValue {
    pub(crate) val: EncryptedValueType,
}

impl EncryptedValue {
    pub fn new(val: EncryptedValueType) -> Self {
        EncryptedValue { val }
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

impl From<i64> for KeyType {
    fn from(value: i64) -> Self {
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

#[cfg(test)]
mod test {
    use ark_std::test_rng;

    use super::KeyType;
    use super::ScalarField;
    use super::{EncryptedValue, EncryptedValueType};
    use ark_ec::Group;
    use ark_std::UniformRand;
    use ark_std::Zero;

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
