//This is used to derive all the account which are required to be process on the solana blockchain

use crate::{constants::ANCHOR_DISCRIMINATOR_SIZE, state::AddressInfo};
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    
    #[account(
        init, 
        payer=authority, 
        space = ANCHOR_DISCRIMINATOR_SIZE + AddressInfo::INIT_SPACE)
    ]
    address_info: Account<'info, AddressInfo>,
    system_program: Program<'info, System>,

}

pub fn create_address_info(
    ctx: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    *ctx.accounts.address_info = AddressInfo {
        name,
        house_number,
        street,
        city,
    };
    Ok(())
}


