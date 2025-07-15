use crate::utils::*;
use crate::state::*;
use anchor_lang::prelude::*;

//This is to initalize the counter
#[derive(Accounts)]
pub struct InitializeCounterContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = ANCHOR_DISCRIMINATOR_SIZE + Counter::INIT_SPACE,
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
