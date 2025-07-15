use anchor_lang::prelude::error_code;

#[error_code]
pub enum CustomError{
    #[msg("This is customer error msg")]
    CustomError,
}