//! Program instruction processor

mod initialize;
mod collect_fees;
mod execute_buyback;
mod burn_tokens;
mod update_params;

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
    error::BurnitError,
    instruction::BurnitInstruction,
};

pub use initialize::process_initialize;
pub use collect_fees::process_collect_fees;
pub use execute_buyback::process_execute_buyback;
pub use burn_tokens::process_burn_tokens;
pub use update_params::process_update_params;

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = BurnitInstruction::try_from_slice(instruction_data)
        .map_err(|_| BurnitError::InvalidInstructionData)?;

    match instruction {
        BurnitInstruction::Initialize { 
            admin_authority, 
            treasury_wallet, 
            team_wallet, 
            fee_collector, 
            token_mint 
        } => {
            msg!("Instruction: Initialize");
            process_initialize(
                program_id, 
                accounts, 
                admin_authority, 
                treasury_wallet, 
                team_wallet, 
                fee_collector, 
                token_mint
            )
        }
        BurnitInstruction::CollectFees { amount } => {
            msg!("Instruction: Collect Fees");
            process_collect_fees(program_id, accounts, amount)
        }
        BurnitInstruction::ExecuteBuyback { amount_in, min_amount_out } => {
            msg!("Instruction: Execute Buyback");
            process_execute_buyback(program_id, accounts, amount_in, min_amount_out)
        }