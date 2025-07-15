use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;
use instructions::*;


declare_id!("A4bAXjdymEQbecR3oVWVscUcM9MZmaE7VCsiWBEvW83A");

#[program]
pub mod pamol_close_account_prj {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, name: String) -> Result<()> {
        create_user::create_user(ctx, name)
    }
    pub fn close_user(ctx: Context<CloseUserContext>) -> Result<()> {
        close_user::close_user(ctx)
    }
}

