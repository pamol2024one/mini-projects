use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DecrementCounterContext<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}