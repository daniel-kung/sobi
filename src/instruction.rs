use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AppInstruction {
    Configure(ConfigureArgs),
    CreateToken(CreateTokenArgs),
    MintToken(MintArgs),
    BurnToken(BurnArgs),
}

pub fn configure(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    args: ConfigureArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Configure(args).try_to_vec().unwrap(),
    })
}

pub fn mint(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    mint_info: &Pubkey,
    token_account: &Pubkey,
    token_info: &Pubkey,
    mint_auth:  &Pubkey,
    token_program_info: &Pubkey, 
    args: MintArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, true),
        AccountMeta::new(*mint_info, true),
        AccountMeta::new(*token_account, false),
        AccountMeta::new(*token_info, false),
        AccountMeta::new(*mint_auth, false),
        AccountMeta::new_readonly(*token_program_info, false),    
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::MintToken(args).try_to_vec().unwrap(),
    })
}

pub fn create_token(
    program_id: &Pubkey,
    siger: &Pubkey,
    token_info: &Pubkey,
    mint: &Pubkey,
    mint_auth:  &Pubkey,
    metadata_key: &Pubkey,
    metadata_program: &Pubkey,
    args: CreateTokenArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*token_info, false),
        AccountMeta::new(*mint, false),
        AccountMeta::new(*mint_auth, false),
        AccountMeta::new(*metadata_key, false),
        AccountMeta::new_readonly(*metadata_program, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::CreateToken(args).try_to_vec().unwrap(),
    })
}

pub fn burn(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    mint_info: &Pubkey,
    token_account: &Pubkey,
    token_info: &Pubkey,
    token_program_info: &Pubkey, 
    args: BurnArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, true),
        AccountMeta::new(*mint_info, true),
        AccountMeta::new(*token_account, false),
        AccountMeta::new(*token_info, false),
        AccountMeta::new_readonly(*token_program_info, false),    
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::BurnToken(args).try_to_vec().unwrap(),
    })
}