// This file will store all the errors message to be generated when those errors happen
use anchor_lang::prelude::error_code;
#[error_code]
pub enum customProjectError {
    #[msg("Generic message occured please check")]
    GenericError,
}