use anchor_lang::prelude::*;

declare_id!("6ShLP1j7ZUMC3V2Y4uL3yqXWpiqmjNFsa1pSH8buftjK");

#[program]
pub mod pamol_hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

//Using this first project to run local using typescript test
