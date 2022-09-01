use anchor_lang::prelude::*;
use anchor_spl::token::{ self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer };
use spl_token::instruction::AuthorityType;

declare_id!("AFLwi1VLdGgtHYmxdg2EeqkYvv2oMWwJE4FpTbQfroL1");

const VESTING_SEED: &[u8] = b"vesting";
const VAULT_SEED: &[u8] = b"vault";
const DAY: u64 = 86400;

#[program]
pub mod vesting {
    use super::*;

    pub fn add_beneficiary(
        ctx: Context<AddBeneficiary>,
        total_amount: u64,
        cliff_days: u64,
        start_days: u64,
        end_days: u64,
        tge_percentage: u64
    ) -> Result<()> {
        if &total_amount < &(&end_days - &start_days) {
            return err!(VestingError::RewardError);
        }
        
        let start_time = (ctx.accounts.clock.unix_timestamp as u64) + (start_days * DAY);
        let end_time = (ctx.accounts.clock.unix_timestamp as u64) + (end_days * DAY);
        let cliff_time = &start_time + (cliff_days * DAY);

        ctx.accounts.vesting_account.beneficiary = ctx.accounts.beneficiary.to_account_info().key();
        ctx.accounts.vesting_account.start_time = start_time;
        ctx.accounts.vesting_account.end_time = end_time;
        ctx.accounts.vesting_account.cliff_time = cliff_time;
        ctx.accounts.vesting_account.owner = ctx.accounts.owner.to_account_info().key();
        ctx.accounts.vesting_account.mint = ctx.accounts.mint.to_account_info().key();
        ctx.accounts.vesting_account.total_vesting_amount = total_amount;
        ctx.accounts.vesting_account.released_amount = 0;
        ctx.accounts.vesting_account.tge_percentage = tge_percentage;
        ctx.accounts.vesting_account.tge_claimed = false;
        ctx.accounts.vesting_account.days_claimed = 0;
        let (vault_authority, _vault_authority_bump) = Pubkey::find_program_address(
            &[VAULT_SEED],
            ctx.program_id
        );

        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority)
        )?;
        token::transfer(ctx.accounts.into_transfer_to_pda_context(), total_amount)?;

        Ok(())
    }

    pub fn claim(ctx: Context<ClaimTokens>) -> Result<()> {
        if
            &ctx.accounts.vesting_account.beneficiary !=
            &ctx.accounts.beneficiary.to_account_info().key()
        {
            return err!(VestingError::InvalidBeneficiary);
        }

        if &(ctx.accounts.clock.unix_timestamp as u64) < &ctx.accounts.vesting_account.start_time {
            return err!(VestingError::VestingNotStarted);
        }

        ctx.accounts.vesting_account.beneficiary_ata = ctx.accounts.beneficiary_ata
            .to_account_info()
            .key();

        let mut claim_amount: u64 = 0;

        if
            &(ctx.accounts.clock.unix_timestamp as u64) >
                &ctx.accounts.vesting_account.start_time &&
            &(ctx.accounts.clock.unix_timestamp as u64) < &ctx.accounts.vesting_account.cliff_time
        {
            if ctx.accounts.vesting_account.tge_claimed == false {
                let mut tge_amount =
                    (&ctx.accounts.vesting_account.total_vesting_amount *
                        &ctx.accounts.vesting_account.tge_percentage) /
                    100;
                claim_amount = tge_amount;
                ctx.accounts.vesting_account.tge_claimed = true;
            }
        } else if
            &(ctx.accounts.clock.unix_timestamp as u64) >
                &ctx.accounts.vesting_account.cliff_time &&
            &(ctx.accounts.clock.unix_timestamp as u64) < &ctx.accounts.vesting_account.end_time
        {
            if ctx.accounts.vesting_account.tge_claimed == false {
                let mut tge_amount =
                    (&ctx.accounts.vesting_account.total_vesting_amount *
                        &ctx.accounts.vesting_account.tge_percentage) /
                    100;
                claim_amount = tge_amount;
                ctx.accounts.vesting_account.tge_claimed = true;
            }

            let total_days =
                (&ctx.accounts.vesting_account.end_time -
                    &ctx.accounts.vesting_account.cliff_time) /
                DAY;

            let daily_amount = &ctx.accounts.vesting_account.total_vesting_amount / &total_days;

            let current_day =
                (&(ctx.accounts.clock.unix_timestamp as u64) -
                    &ctx.accounts.vesting_account.cliff_time) /
                DAY;

            let unpaid_days = &(current_day as u64) - &ctx.accounts.vesting_account.days_claimed;

            claim_amount += unpaid_days * daily_amount;

            ctx.accounts.vesting_account.days_claimed = current_day;
        } else {
            if ctx.accounts.vesting_account.tge_claimed == false {
                let tge_amount =
                    (&ctx.accounts.vesting_account.total_vesting_amount *
                        &ctx.accounts.vesting_account.tge_percentage) /
                    100;
                claim_amount = tge_amount;
                ctx.accounts.vesting_account.tge_claimed = true;
                ctx.accounts.vesting_account.released_amount += &claim_amount;
                msg!("3, {}", &claim_amount);
            }
            let left_amount =
                &ctx.accounts.vesting_account.total_vesting_amount -
                &ctx.accounts.vesting_account.released_amount;

            claim_amount += left_amount;
        }

        ctx.accounts.vesting_account.released_amount += &claim_amount;

        let (_vault_authority, vault_authority_bump) = Pubkey::find_program_address(
            &[VAULT_SEED],
            ctx.program_id
        );

        let authority_seeds = &[&VAULT_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_beneficiary_context()
                .with_signer(&[&authority_seeds[..]]),
            claim_amount
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddBeneficiary<'info> {
    ///CHECK
    #[account(mut,signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub owner_ata: Account<'info, TokenAccount>,
    ///CHECK
    pub beneficiary: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        seeds = [VAULT_SEED, beneficiary.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = owner
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        seeds = [VESTING_SEED, beneficiary.key().as_ref()],
        bump,
        space = 200
    )]
    pub vesting_account: Box<Account<'info, VestingAccount>>,
    ///CHECK
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    ///CHECK
    pub token_program: AccountInfo<'info>,
}
#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    ///CHECK
    #[account(mut,signer)]
    pub beneficiary: AccountInfo<'info>,
    #[account(mut)]
    pub beneficiary_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            VAULT_SEED,
            beneficiary.key().as_ref()
        ],
        bump
    )]
    ///CHECK
    pub vault_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [
            VESTING_SEED,
            beneficiary.key().as_ref()
        ],
        bump
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    ///CHECK
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    ///CHECK
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct VestingAccount {
    pub beneficiary: Pubkey,
    pub beneficiary_ata: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub cliff_time: u64,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub total_vesting_amount: u64,
    pub released_amount: u64,
    pub tge_percentage: u64,
    pub tge_claimed: bool,
    pub days_claimed: u64,
}

#[error_code]
pub enum VestingError {
    // 1
    #[msg("daily reward rate should be greater than 1 ")]
    RewardError,

    //2
    #[msg("Vesting period is not started")]
    VestingNotStarted,

    //3
    #[msg("Invalid beneficiary account")]
    InvalidBeneficiary,
}

impl<'info> AddBeneficiary<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.owner.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.owner_ata.to_account_info().clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.owner.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> ClaimTokens<'info> {
    fn into_transfer_to_beneficiary_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.beneficiary_ata.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}