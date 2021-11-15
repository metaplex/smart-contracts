//! Clean up pack config account

use crate::{
    find_pack_config_program_address,
    state::{PackConfig, CleanUpActions,},
    utils::*,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::Pack,
    pubkey::Pubkey,
};

/// Process CleanUp instruction
pub fn clean_up(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pack_set_info = next_account_info(account_info_iter)?;
    let pack_config_info = next_account_info(account_info_iter)?;

    // check if pack config is correct
    let (pack_config_pubkey, _) =
        find_pack_config_program_address(program_id, pack_set_info.key);
    assert_account_key(pack_config_info, &pack_config_pubkey)?;
    let mut pack_config = PackConfig::unpack(&pack_config_info.data.borrow())?;

    match pack_config.action_to_do {
        CleanUpActions::Change(card_index, new_value) => {
            if new_value == 0 {
                pack_config.remove_at(card_index);
            } else {
                pack_config.change(card_index, new_value)?;
            }

            PackConfig::pack(pack_config, *pack_config_info.data.borrow_mut())?;

            Ok(())
        }
        CleanUpActions::Sort => {
            pack_config.sort();

            PackConfig::pack(pack_config, *pack_config_info.data.borrow_mut())?;

            Ok(())
        }
        CleanUpActions::None => {
            Ok(())
        }
    }
}