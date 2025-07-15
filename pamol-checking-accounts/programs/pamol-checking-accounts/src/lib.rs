use anchor_lang::prelude::*;

declare_id!("CAqotcekwXCRq2iWSDC6pSAoNHzhc7ptujM9MNQQofgT");

#[program]
pub mod pamol_checking_accounts {
    use super::*;

    pub fn check_accounts(ctx: Context<CheckAccounts>) -> Result<()> {
        msg!("Checking accounts for : {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckAccounts<'info> {
    //check account is the signer
    payer: Signer<'info>,

    //Creating an account with no checking 
    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,

    //Creating an account with owner check 
    #[account(mut, owner = id())]
    account_to_change: UncheckedAccount<'info>,

    //Check account is the executable and is the system program
    system_program: Program<'info, System>,

}
