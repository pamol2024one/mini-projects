use anchor_lang::prelude::*;

declare_id!("A4bAXjdymEQbecR3oVWVscUcM9MZmaE7VCsiWBEvW83A");

#[program]
pub mod pamol_close_account_prj {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
