//! BURNIT Protocol - Automated Buyback & Burn System

mod error;
mod instruction;
mod processor;
mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current SDK types for downstream users.
pub use error::BurnitError;
pub use instruction::BurnitInstruction;
pub use state::{BurnitState, MAX_PROTOCOL_FEE, FEE_DENOMINATOR};

solana_program::declare_id!("BURNxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");