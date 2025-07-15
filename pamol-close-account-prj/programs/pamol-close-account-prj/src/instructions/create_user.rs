use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = UserState::INIT_SPACE,
        seeds =[USER_SEED.as_bytes(), user.key().as_ref(),],
        bump,
    )]
    pub user_account: Account<'info, UserState>,
    pub system_program: Program<'info, System>
}

pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()>{
    *ctx.accounts.user_account = UserState {
        bump: ctx.bumps.user_account,
        user: ctx.accounts.user.key(),
        name,
    };
    Ok(())
}