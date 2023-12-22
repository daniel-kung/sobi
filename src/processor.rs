use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::{instruction::*, state::MintArgs};

pub mod configure;
pub use configure::*;

pub mod mint;
pub use mint::*;

pub mod burn;
pub use burn::*;

pub mod create_token;
pub use create_token::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = AppInstruction::try_from_slice(input)?;
    match instruction {
        AppInstruction::Configure(args) => {
            msg!("Instruction: Configure");
            process_configure(program_id, accounts, args)
        }
        AppInstruction::CreateToken(args) => {
            msg!("Instruction: CreateToken");
            process_create_token(program_id, accounts, args)
        }
        AppInstruction::MintToken(args) => {
            msg!("Instruction: Mint");
            process_mint(program_id, accounts,args)
        }
        AppInstruction::BurnToken(args) => {
            msg!("Instruction: Burn");
            process_burn(program_id, accounts,args)
        }
        
    }
}
