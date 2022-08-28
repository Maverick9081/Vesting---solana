use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct VestingAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub cliff_time: i64,
    pub duration: i64,
    pub vester: Pubkey,
    pub mint: Pubkey,
    pub total_vesting_amount: u64,
    pub released_amount: u64,
}
