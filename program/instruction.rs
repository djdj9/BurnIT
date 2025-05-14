//! Instruction types for BURNIT Protocol

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar,
};

/// Instructions supported by the BURNIT Protocol
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub enum BurnitInstruction {
    /// Initialize the BURNIT protocol
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Authority initializing the program
    /// 1. `[writable]` Burnit state account
    /// 2. `[]` Admin authority
    /// 3. `[]` Treasury wallet
    /// 4. `[]` Team wallet
    /// 5. `[]` Fee collector wallet
    /// 6. `[]` Token mint
    /// 7. `[]` System program
    /// 8. `[]` Rent sysvar
    Initialize {
        /// Admin authority that can update parameters
        admin_authority: Pubkey,
        
        /// Treasury wallet to receive portion of fees
        treasury_wallet: Pubkey,
        
        /// Team wallet to receive portion of fees
        team_wallet: Pubkey,
        
        /// Fee collector wallet
        fee_collector: Pubkey,
        
        /// Token mint
        token_mint: Pubkey,
    },

    /// Collect fees from DEX swaps
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Fee authority
    /// 1. `[writable]` Burnit state account
    /// 2. `[writable]` Fee source account
    /// 3. `[writable]` Fee collector account
    /// 4. `[]` System program
    CollectFees {
        /// Amount of fees to collect (in lamports)
        amount: u64,
    },

    /// Execute buyback using collected fees
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Buyback authority
    /// 1. `[writable]` Burnit state account
    /// 2. `[writable]` Fee collector account
    /// 3. `[writable]` Token account to receive purchased tokens
    /// 4. `[]` DEX program account
    /// 5. `[writable]` DEX market account
    /// 6. `[writable]` DEX request queue account
    /// 7. `[writable]` DEX event queue account
    /// 8. `[writable]` DEX bids account
    /// 9. `[writable]` DEX asks account
    /// 10. `[writable]` DEX base vault account
    /// 11. `[writable]` DEX quote vault account
    /// 12. `[]` Token program
    /// 13. `[]` System program
    ExecuteBuyback {
        /// Amount of SOL to use for buyback (in lamports)
        amount_in: u64,
        
        /// Minimum amount of tokens to receive
        min_amount_out: u64,
    },

    /// Burn tokens from the buyback wallet
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Burn authority
    /// 1. `[writable]` Burnit state account
    /// 2. `[writable]` Token account holding tokens to burn
    /// 3. `[writable]` Token mint
    /// 4. `[]` Token program
    /// 5. `[]` System program
    BurnTokens {
        /// Amount of tokens to burn
        amount: u64,
    },

    /// Update program parameters
    /// 
    /// Accounts expected:
    /// 0. `[signer]` Admin authority
    /// 1. `[writable]` Burnit state account
    UpdateParams {
        /// New protocol fee (in basis points)
        fee_bps: Option<u16>,
        
        /// New buyback allocation (in basis points)
        buyback_allocation_bps: Option<u16>,
        
        /// New treasury allocation (in basis points)
        treasury_allocation_bps: Option<u16>,
        
        /// New team allocation (in basis points)
        team_allocation_bps: Option<u16>,
        
        /// New max price impact (in basis points)
        max_price_impact_bps: Option<u16>,
        
        /// Toggle auto-burn feature
        auto_burn_enabled: Option<bool>,
    },
}