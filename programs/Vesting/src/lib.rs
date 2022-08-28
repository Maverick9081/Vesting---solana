use anchor_lang::prelude::*;
use anchor_spl::token::{ self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer };
// use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const VESTING_SEED: &[u8] = b"vesting";
     const VAULT_SEED: &[u8] = b"vault";

#[program]
pub mod vesting {
    use super::*;

     

    pub fn add_beneficiary(ctx: Context<AddBeneficiary>) -> Result<()> {
        Ok(())
    }

    // pub fn claim(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // } 
}



#[derive(Accounts)]
pub struct AddBeneficiary<'info> {
    ///CHECK
    #[account(mut,signer)]
    pub owner : AccountInfo<'info>,
     ///CHECK
    pub beneficiary : AccountInfo<'info>,
    pub beneficiary_ata : Account <'info,TokenAccount>,
    pub mint : Account<'info,Mint>,
    #[account(
        init,
        payer = owner,
        seeds = [
            VAULT_SEED.as_ref(),
            beneficiary.key().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = owner
    )]
    pub vault_account : Account<'info,TokenAccount>,
    #[account(
        init,
        payer = owner,
        seeds = [
            VESTING_SEED,
            beneficiary.key().as_ref()
        ],
        bump,
        space = 90
    )]
    pub vesting_account : Account<'info,VestingAccount>,
     ///CHECK
    pub system_program : AccountInfo<'info>,
    pub rent : Sysvar<'info,Rent>,
     ///CHECK
    pub token_program :  AccountInfo<'info>
}

#[account]
pub struct VestingAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub cliff_time: i64,
    pub duration: i64,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub total_vesting_amount: u64,
    pub released_amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum Roles {
    Advisor,
    Partner,
    Mentor
}