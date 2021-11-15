//! Claim pack instruction processing

use crate::{
    error::NFTPacksError,
    find_pack_card_program_address, find_program_authority,
    math::SafeMath,
    state::{
        PackCard, PackDistributionType, PackSet, ProvingProcess,
        PREFIX,
    },
    utils::*,
};
use metaplex_token_metadata::state::{MasterEditionV2, Metadata};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::Account;

/// Process ClaimPack instruction
pub fn claim_pack(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pack_set_account = next_account_info(account_info_iter)?;
    let proving_process_account = next_account_info(account_info_iter)?;
    let user_wallet_account = next_account_info(account_info_iter)?;
    let user_voucher_token_account = next_account_info(account_info_iter)?;
    let program_authority_account = next_account_info(account_info_iter)?;
    let pack_card_account = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let new_metadata_account = next_account_info(account_info_iter)?;
    let new_edition_account = next_account_info(account_info_iter)?;
    let master_edition_account = next_account_info(account_info_iter)?;
    let new_mint_account = next_account_info(account_info_iter)?;
    let new_mint_authority_account = next_account_info(account_info_iter)?;
    let metadata_account = next_account_info(account_info_iter)?;
    let metadata_mint_account = next_account_info(account_info_iter)?;
    let edition_marker_account = next_account_info(account_info_iter)?;
    let rent_account = next_account_info(account_info_iter)?;
    let randomness_oracle_account = next_account_info(account_info_iter)?;
    let _token_metadata_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;
    let _rent = &Rent::from_account_info(rent_account)?;

    // Validate owners
    assert_owned_by(pack_set_account, program_id)?;
    assert_owned_by(randomness_oracle_account, &randomness_oracle_program::id())?;

    assert_signer(&user_wallet_account)?;

    let pack_set = PackSet::unpack(&pack_set_account.data.borrow())?;
    let mut proving_process = ProvingProcess::unpack(&proving_process_account.data.borrow_mut())?;
    let index = proving_process.next_card_to_redeem;

    assert_account_key(pack_set_account, &proving_process.pack_set)?;

    let user_token_acc = Account::unpack(&user_voucher_token_account.data.borrow_mut())?;
    if user_token_acc.mint != proving_process.voucher_mint {
        return Err(NFTPacksError::WrongEditionMint.into());
    }
    if user_token_acc.owner != *user_wallet_account.key {
        if let COption::Some(delegated) = user_token_acc.delegate {
            if user_token_acc.delegated_amount == 0 || delegated != *user_wallet_account.key {
                return Err(NFTPacksError::WrongVoucherOwner.into());
            }
        } else {
            return Err(NFTPacksError::WrongVoucherOwner.into());
        }
    }

    // Validate PackCard
    let (valid_pack_card, _) =
        find_pack_card_program_address(program_id, pack_set_account.key, index);
    assert_account_key(pack_card_account, &valid_pack_card)?;

    let mut pack_card = PackCard::unpack(&pack_card_account.data.borrow())?;
    assert_account_key(pack_set_account, &pack_card.pack_set)?;

    // Obtain master metadata instance
    let master_metadata = Metadata::from_account_info(metadata_account)?;

    let master_edition = MasterEditionV2::from_account_info(master_edition_account)?;

    // Check metadata mint
    assert_account_key(metadata_mint_account, &master_metadata.mint)?;

    let (program_authority_key, bump_seed) = find_program_authority(program_id);
    assert_account_key(program_authority_account, &program_authority_key)?;

    if proving_process.cards_redeemed == pack_set.allowed_amount_to_redeem {
        return Err(NFTPacksError::UserRedeemedAllCards.into());
    }

    // set value to 0 so user can't redeem same card twice and can't redeem any card
    proving_process.next_card_to_redeem = 0;

    if pack_set.distribution_type != PackDistributionType::Unlimited {
        pack_card.decrement_supply()?;
    }

    // Mint token
    spl_token_metadata_mint_new_edition_from_master_edition_via_token(
        new_metadata_account,
        new_edition_account,
        new_mint_account,
        new_mint_authority_account,
        user_wallet_account,
        program_authority_account,
        user_token_account,
        metadata_account,
        master_edition_account,
        metadata_mint_account,
        edition_marker_account,
        token_program_account,
        system_program_account,
        rent_account,
        master_edition.supply.error_increment()?,
        &[PREFIX.as_bytes(), program_id.as_ref(), &[bump_seed]],
    )?;

    proving_process.cards_redeemed = proving_process.cards_redeemed.error_increment()?;

    // Update state
    ProvingProcess::pack(proving_process, *proving_process_account.data.borrow_mut())?;
    PackCard::pack(pack_card, *pack_card_account.data.borrow_mut())?;

    Ok(())
}
