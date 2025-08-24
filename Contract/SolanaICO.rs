use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("PROGRAM_ID");

#[error_code]
pub enum ErrorCode{
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Invalid admin")]
    InvalidAdmin,
}

#[program]
pub mod ico {
    pub const ICO_MINT_ADDRESS: &str = "";
    pub const LAMPORTS_PER_TOKEN: u64 = 1_000_000; // 0.001 SOL
    pub const TOKEN_DECIMALS: u64 = 1_000_000_000; // 10^9 for SPL token decimals

    use super::*;

    // initialize and deposite initialize tokens
    pub fn create_ico_ata(ctx: Context<CreateIcoATA>, ico_amount: u64) -> Result<()> {
        msg!("Creating program ATA to hold ICO Tokens")
        //
    }

    // add more tokens
    pub fn deposite_ico_in_ata() {}

    pub fn buy_tokens() {}

    // to store the data
    #[derive(Accounts)]
    pub struct CreateIcoATA<'info> {}

    #[derive(Accounts)]
    pub struct DepositeIcoATA<'info> {}

    #[derive(Accounts)]
    #[instruction(_ico_ata_for_ico_program_bumo: u8)]
    pub struct BuyTokens<'info>{}

    #[account]
    pub struct Data {}
}
