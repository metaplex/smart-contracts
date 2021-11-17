//! Proving process definitions

use super::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::collections::BTreeMap;

/// Proving process
#[repr(C)]
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize, Default)]
pub struct ProvingProcess {
    /// Account type - ProvingProcess
    pub account_type: AccountType,
    /// Voucher mint
    pub voucher_mint: Pubkey,
    /// Pack set
    pub pack_set: Pubkey,
    /// BTreeMap with cards to redeem and statuses if it's already redeemed
    pub cards_to_redeem: BTreeMap<u32, bool>,
}

impl ProvingProcess {
    /// Prefix used to generate account
    pub const PREFIX: &'static str = "proving";

    /// Amount of tokens for prove operation
    pub const TOKEN_AMOUNT: u64 = 1;

    /// One BTreeMap element len
    pub const ONE_ELEMENT_LEN: usize = 5;

    /// Initialize a ProvingProcess
    pub fn init(&mut self, params: InitProvingProcessParams) {
        self.account_type = AccountType::ProvingProcess;
        self.voucher_mint = params.voucher_mint;
        self.pack_set = params.pack_set;
        self.cards_to_redeem = BTreeMap::new();
    }
}

/// Initialize a ProvingProcess params
pub struct InitProvingProcessParams {
    /// Voucher mint
    pub voucher_mint: Pubkey,
    /// Pack set
    pub pack_set: Pubkey,
}

impl Sealed for ProvingProcess {}

impl Pack for ProvingProcess {
    // 1 + 32 + 32 + BTreeMap size
    // base account len without BTreeMap such as it's dynamic size
    const LEN: usize = 65;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(src).map_err(|_| {
            msg!("Failed to deserialize");
            ProgramError::InvalidAccountData
        })
    }
}

impl IsInitialized for ProvingProcess {
    fn is_initialized(&self) -> bool {
        self.account_type != AccountType::Uninitialized
            && self.account_type == AccountType::ProvingProcess
    }
}
