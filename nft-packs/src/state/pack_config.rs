//! Pack config definitions

use super::*;
use crate::math::SafeMath;
use std::collections::BTreeMap;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// Pack config. PDA (["config", pack_key], program_id)
#[repr(C)]
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PackConfig {
    /// Account type - PackConfig
    pub account_type: AccountType,
    /// weights; BTreeMap<u32 card_index, u32 either max_supply or weight>
    pub weights: BTreeMap<u32, u32>,
}

impl PackConfig {
    /// Prefix used to generate account
    pub const PREFIX: &'static str = "config";

    /// Initialize a PackConfig
    pub fn init(&mut self) {
        self.account_type = AccountType::PackConfig;
        self.weights = BTreeMap::new();
    }
}

impl Sealed for PackConfig {}

impl Pack for PackConfig {
    // TODO: check if we can hold more
    /// Max size of config to hold max allowed amount of cards. 50 ?
    /// 1+(12*50)
    const LEN: usize = 601;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut src_mut = src;
        Self::deserialize(&mut src_mut).map_err(|_| {
            msg!("Failed to deserialize");
            ProgramError::InvalidAccountData
        })
    }
}

impl IsInitialized for PackConfig {
    fn is_initialized(&self) -> bool {
        self.account_type != AccountType::Uninitialized
            && self.account_type == AccountType::PackConfig
    }
}