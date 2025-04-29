//! # revm-primitives
//!
//! EVM primitive types.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

pub mod constants;
pub mod eip170;
pub mod eip4844;
pub mod eip7702;
pub mod eof;
pub mod hardfork;

pub use constants::*;

// Reexport alloy primitives.

pub use alloy_primitives::map::{self, hash_map, hash_set, HashMap, HashSet};
pub use alloy_primitives::{
    self, address, b256, bytes, fixed_bytes, hex, hex_literal, keccak256, ruint, uint, Address,
    Bytes, FixedBytes, Log, LogData, TxKind, B256, I128, I256, U128, U256,
};

/// type alias for storage keys
pub type StorageKey = U256;
/// type alias for storage values
pub type StorageValue = FlaggedStorage;



use alloy_primitives::ruint::UintTryFrom;

#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, core::fmt::Debug, Hash)]
/// A storage value with a flag to indicate whether it is private or public
pub struct FlaggedStorage {
    /// The underlying value in the storage slot
    pub word: U256,
    /// Whether the value is private or public
    pub is_private: bool,
}

impl<T> From<T> for FlaggedStorage
where
    U256: UintTryFrom<T>,
{
    fn from(value: T) -> Self {
        Self { word: U256::from(value), is_private: false }
    }
}

impl From<FlaggedStorage> for FixedBytes<32> {
    fn from(storage: FlaggedStorage) -> FixedBytes<32> {
        FixedBytes::<32>::from(storage.word)
    }
}

impl From<FlaggedStorage> for U256 {
    fn from(storage: FlaggedStorage) -> U256 {
        storage.word
    }
}

impl From<&FlaggedStorage> for U256 {
    fn from(storage: &FlaggedStorage) -> U256 {
        storage.word
    }
}

impl FlaggedStorage {
    /// The default word for a flagged storage slot
    /// when no state has been set. Importantly, this slot is public by default
    pub const ZERO: Self = Self { word: U256::ZERO, is_private: false };

    /// create a new flagged storage word
    pub fn new<T>(word: T, is_private: bool) -> Self
    where
        U256: UintTryFrom<T>,
    {
        Self { word: U256::from(word), is_private }
    }

    /// create a new flagged storage word from a tuple
    pub fn new_from_tuple<T>((word, is_private): (T, bool)) -> Self
    where
        U256: UintTryFrom<T>,
    {
        Self { word: U256::from(word), is_private }
    }

    /// create a new flagged storage word from a word
    /// defaults to private
    pub fn new_from_word<T>(word: T) -> Self
    where
        U256: UintTryFrom<T>,
    {
        Self {
            word: U256::from(word),
            is_private: false, // Default to false
        }
    }

    /// returns whether the word is private
    pub fn is_private(&self) -> bool {
        self.is_private
    }

    /// returns whether the word is public
    pub fn is_public(&self) -> bool {
        !self.is_private
    }

    /// sets the visibility of the word
    pub fn set_visibility(&self, is_private: bool) -> Self {
        FlaggedStorage { word: self.word, is_private }
    }

    /// Sets the private flage to true
    pub fn mark_private(&self) -> Self {
        self.set_visibility(true)
    }

    /// sets the private flage to false
    pub fn mark_public(&self) -> Self {
        self.set_visibility(false)
    }

    /// Returns whether the word is the ZERO default word
    /// True if the word is zero and is flagged as publi
    pub fn is_zero(&self) -> bool {
        self.is_public() && self.word.is_zero()
    }
}