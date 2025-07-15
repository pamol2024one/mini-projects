use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;
use instructions::*;

declare_id!("FBD4hBi3NGrmAzYx1XQukBvESFB8PzzdnhmvNmZfnhhR");

#[program]
pub mod pamol_counter_prj {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounterContext>) -> Result<()> {
        msg!("Greetings Counter has been initialized: {:?}", 0);
        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounterContext>) -> Result<()>{
        let counter_increase = &mut ctx.accounts.counter;
        counter_increase.count = counter_increase.count.checked_add(1).unwrap();
        Ok(())
    }
    pub fn decrement_counter(ctx: Context<DecrementCounterContext>) -> Result<()>{
        let counter_decrease = &mut ctx.accounts.counter;
        counter_decrease.count = counter_decrease.count.checked_sub(1).unwrap();
        Ok(())
    }
}

