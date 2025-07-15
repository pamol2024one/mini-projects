//Import packages and specific funtions to run the project code which are external to project
use anchor_lang::prelude::*;

//This is the program hash which is needed to identify the when running it on solana blockchain 
declare_id!("E95EGJhRievFCk1u3DFuhRkN9uDUEERs3HVxFF1uWPeu");

//Make a files location aware to the program
pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;
//Import the project items which are stored in different files accessible here
use crate::{constants::*, errors::*, state::*, instructions::*};

#[program]
pub mod pamol_account_data_prj {
    use super::*;

    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        create_data::create_address_info(ctx, name, house_number, street, city)
        // Ok(())
    }
}



