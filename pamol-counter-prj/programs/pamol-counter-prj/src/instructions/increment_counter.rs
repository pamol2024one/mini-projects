use crate::state::*;
use anchor_lang::prelude::*;

//This is to increment the counter
#[derive(Accounts)]
pub struct IncrementCounterContext<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}