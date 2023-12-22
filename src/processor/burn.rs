use crate::{ferror, state::*, utils::*};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

pub fn process_burn(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BurnArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let token_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_config(&program_id, &config_info)?;
    let config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&signer_info, &config_data.authority)?;

    assert_token_info(program_id, &mint_info.key, token_info)?;

    let mut token_data = TokenData::from_account_info(token_info)?;
    let multiplier = (10 as u64).pow(token_data.decimals as u32) as u64;
    let amt = args.amt.checked_mul(multiplier).ok_or(ProgramError::BorshIoError("mint amt error".into()))?;
    spl_token_burn(
        token_program_info,
        mint_info,
        token_account,
        signer_info,
        rent_info,
        amt
    )?;
    
    token_data.supply -= amt;
    token_data.serialize(&mut *token_info.try_borrow_mut_data()?)?;
    Ok(())
}
