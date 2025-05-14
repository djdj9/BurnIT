//! State definitions for the BURNIT Protocol

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};

/// Basis points denominator - 10000 equals 100%
pub const FEE_DENOMINATOR: u16 = 10000;

/// Maximum protocol fee in basis points (10% = 1000)
pub const MAX_PROTOCOL_FEE: u16 = 1000;

/// Default protocol fee in basis points (0.3% = 30)
pub const DEFAULT_PROTOCOL_FEE: u16 = 30;

/// Buyback & Burn parameters
pub const MIN_SOL_AMOUNT_FOR_BUYBACK: u64 = 10_000_000; // 0.01 SOL
pub const MAX_PRICE_IMPACT_BPS: u16 = 300; // Max 3% price impact
pub const AUTO_BURN_ENABLED: bool = true;
pub const BURN_COOLDOWN_SECONDS: u64 = 86400; // 24 hours

/// Fee allocation defaults
pub const BUYBACK_ALLOCATION_BPS: u16 = 5000; // 50% of fees go to buyback

/// Program state data structure
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BurnitState {
    /// Is this state initialized
    pub is_initialized: bool,
    
    /// Admin authority that can update parameters
    pub admin_authority: Pubkey,
    
    /// Treasury wallet to receive portion of fees
    pub treasury_wallet: Pubkey,
    
    /// Team wallet to receive portion of fees
    pub team_wallet: Pubkey,
    
    /// Fee collector wallet that accumulates fees before distribution
    pub fee_collector: Pubkey,
    
    /// Token mint address for the BURNIT token
    pub token_mint: Pubkey,
    
    /// PDA with authority to burn tokens
    pub token_burn_authority: Pubkey,
    
    /// Bump seed for PDA derivation
    pub burn_authority_bump_seed: u8,
    
    /// Total fees collected (in lamports)
    pub total_fees_collected: u64,
    
    /// Total tokens burned
    pub total_tokens_burned: u64,
    
    /// Timestamp of last buyback
    pub last_buyback_timestamp: u64,
    
    /// Timestamp of last burn
    pub last_burn_timestamp: u64,
    
    /// Protocol fee in basis points (e.g. 30 = 0.3%)
    pub fee_bps: u16,
    
    /// Percentage of fees allocated to buyback (in bps)
    pub buyback_allocation_bps: u16,
    
    /// Percentage of fees allocated to treasury (in bps)
    pub treasury_allocation_bps: u16,
    
    /// Percentage of fees allocated to team (in bps)
    pub team_allocation_bps: u16,
    
    /// Maximum price impact for buybacks (in bps)
    pub max_price_impact_bps: u16,
    
    /// Whether auto-burn is enabled
    pub auto_burn_enabled: bool,
}

impl IsInitialized for BurnitState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl BurnitState {
    /// Get the size of the state struct for account allocation
    pub const fn get_packed_len() -> usize {
        // Calculate size based on struct fields
        // This is a simplified calculation
        208
    }
    
    /// Validate fee allocations add up to 100%
    pub fn validate_fee_allocations(&self) -> Result<(), ProgramError> {
        let total_allocation = self.buyback_allocation_bps
            .saturating_add(self.treasury_allocation_bps)
            .saturating_add(self.team_allocation_bps);
            
        if total_allocation != FEE_DENOMINATOR {
            return Err(ProgramError::InvalidArgument);
        }
        
        Ok(())
    }
}