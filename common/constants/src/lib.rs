#![no_std]

/// Maximum percentage value (100% = 10000, 2 decimal places)
pub static MAX_PERCENTAGE: u64 = 10_000;

/// Cost to issue a new ESDT token (in eGLD)
pub static TOKEN_ISSUANCE_COST: u64 = 50_000_000_000_000_000;

pub static WAD_DECIMALS: usize = 18;