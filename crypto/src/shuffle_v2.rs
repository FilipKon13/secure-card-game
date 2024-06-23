use crate::encryption::{decrypt, encrypt, rand_key};
use crate::types::{EncryptedValue, KeyType};
use rand::{seq::SliceRandom, thread_rng};
