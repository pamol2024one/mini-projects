//This is where struct are defined which are instantiated to store data
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64, // 4 bytes
}