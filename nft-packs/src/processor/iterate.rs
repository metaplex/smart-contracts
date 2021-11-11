//! Add card to pack instruction processing

use crate::{
    error::NFTPacksError,
    find_pack_card_program_address, find_pack_config_program_address, find_program_authority,
    instruction::AddCardToPackArgs,
    math::SafeMath,
    state::{
        InitPackCardParams, PackCard, PackConfig, PackDistributionType, PackSet, PackSetState,
        MAX_PACK_CARDS_AMOUNT,
    },
    utils::*,
};
use metaplex::state::Store;
use metaplex_token_metadata::{
    error::MetadataError,
    state::{MasterEditionV2, Metadata, EDITION, PREFIX},
    utils::{assert_derivation, assert_initialized},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::Account;

/// Process Iterate instruction
pub fn iterate(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pack_set_info = next_account_info(account_info_iter)?;
    let pack_config_info = next_account_info(account_info_iter)?;

    // iterate deserialized
    let mut pack_config = PackConfig::unpack(&pack_config_info.data.borrow_mut())?;
    pack_config.sort();
    for el in pack_config.weights.iter() {
        msg!("{:?}", el);
    }

    // iterate raw bytes
    // TODO
    // 112_718 of 200_000

    Ok(())
}
