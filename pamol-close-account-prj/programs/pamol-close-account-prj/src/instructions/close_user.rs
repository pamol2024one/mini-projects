use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseUserContext<'info>{
    //Get the account who will pay to process
    pub user: Signer<'info>,
    //Identify the account which needs to be deleted i.e. closed
    #[account(
        mut,
        seeds =[USER_SEED.as_bytes(), user.key().as_ref(),],
        bump = user_account.bump,
        close = user, //close account and return the lamport to user
    )]
    pub user_account: Account<'info, UserState>,
}

pub fn close_user(_ctx: Context<CloseUserContext>) -> Result<()>{
    Ok(())
}