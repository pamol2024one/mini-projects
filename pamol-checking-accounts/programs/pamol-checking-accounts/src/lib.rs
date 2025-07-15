use anchor_lang::prelude::*;

declare_id!("CAqotcekwXCRq2iWSDC6pSAoNHzhc7ptujM9MNQQofgT");

#[program]
pub mod pamol_checking_accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
