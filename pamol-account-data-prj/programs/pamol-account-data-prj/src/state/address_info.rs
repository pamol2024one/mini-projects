//This is where the data struct are defined to be used in the project
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] //automatically calculate the space required for this struct and 
//store in INIT_SPACE const
pub struct AddressInfo {
    #[max_len(50)]
    pub name: String, //4 bytes + 50 bytes
    pub house_number: u8, // 1 byte
    #[max_len(50)]
    pub street: String, //4 bytes + 50 bytes
    #[max_len(50)]
    pub city: String // 4 bytes + 50 bytes
}