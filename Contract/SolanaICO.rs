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
        // Convert amount to token decimals
        let raw_amount = ico_amount
            .checked_mul(TOKEN_DECIMALS)
            .ok_or(ErrorCode::Overflow)

        let cpi_ctx = CpiContext::new(
            ctx.acconts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.acconts.ico_ata_for_admin.to_account_info(),
                to: ctx.acconts._ico_ata_for_ico_program.to_account_info(),
                authority: ctx.acconts.admin.to_account_info(),
            }
        )

        token::Transfer(cpi_ctx, raw_amount)?;
        msg!("Transfered {} ICO tokens to program ATA", ico_amount);

        // update our state variable
        let data = &mut ctx.acconts.data;
        data.admin = *ctx.acconts.admin.key;
        data.total_tokens = ico_amount;
        data.tokens_sold = 0;
        msg!("Initialized ICO data");
        Ok(())
    }

    // add more tokens
    pub fn deposite_ico_in_ata(ctx: Context<DepositeIcoATA>, ico_amount: u64) -> Result<()> {
        if ctx.acconts.data.admin != *ctx.acconts.admin.key {
            return Err(error!(ErrorCode::InvalidAdmin))
        }

        // Convert amount to token decimals
        let raw_amount = ico_amount
            .checked_mul(TOKEN_DECIMALS)
            .ok_or(ErrorCode::Overflow)

        let cpi_ctx = CpiContext::new(
            ctx.acconts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.acconts.ico_ata_for_admin.to_account_info(),
                to: ctx.acconts._ico_ata_for_ico_program.to_account_info(),
                authority: ctx.acconts.admin.to_account_info(),
            }
        )

        token::Transfer(cpi_ctx, raw_amount)?;

        // update our state variable
        let data = &mut ctx.acconts.data;
        data.total_tokens += ico_amount;
        msg!("Deposite {} additional ICO tokens", ico_amount);
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, _ico_ata_for_ico_program_bump: u8, token_amount: u64) -> Result<()> {
        // Convert token amount to incude decimals for SPL transfer
        let raw_token_amount = token_amount
            .checked_mul(TOKEN_DECIMALS)
            .ok_or(ErrorCode::Overflow)

        let sol_amount = token_amount
            .checked_mul(LAMPORTS_PER_TOKEN)
            .ok_or(ErrorCode::Overflow)

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.acconts.user.key(),
            &ctx.acconts.admin.key(),
            sol_amount,
        )

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.acconts.user.to_account_info(),
                ctx.acconts.admin.to_account_info()
            ],
        )?;

        msg!("Transferred {} lamports to admin", sol_amount);

        // TRANSFER TOKEN TO USER
        let ico_mint_address = ctx.acconts.ico_mint.key();
        let seeds = &[ico_mint_address.as_ref(), &[_ico_ata_for_ico_program_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.acconts.token_program.to_account_info(),
            token::Transfer{
                from: ctx.acconts._ico_ata_for_ico_program.to_account_info(),
                to: ctx.acconts.ico_ata_for_user.to_account_info(),
                authority: ctx.acconts._ico_ata_for_ico_program.to_account_info(),
            },
            &signer,
        );

        token::transfer(cpi_ctx, raw_token_amount)?;

        // UPDATE DATA
        let data = &mut ctx.acconts.data;
        data.tokens_sold = data
            .tokens_sold
            .checked_add(token_amount)
            .ok_or(ErrorCode::Overflow)?;

        msg!("Transferred {} token to buyer", token_amount);
        Ok(())
    }

    // to store the data
    #[derive(Accounts)]
    pub struct CreateIcoATA<'info> {
        #[account(
            init,
            payer = admin,
            seeds = [ ICO_MINT_ADDRESS.parsed::<Pubkey>().unwrap().as_ref() ],
            bump,
            token::mint = ico_mint,
            token::authority = ico_ata_for_ico_program,
        )]
        pub ico_ata_for_ico_program: Account<'info, TokenAccount>,

        #[account(
            init,
            payer = admin,
            space = 9000,
            seeds = [b"data", admin.key().as_ref()],
            bump
        )]
        pub data: Account<'info, Data>,

        #[account(
            address = ICO_MINT_ADDRESS.parse::<Pubkey>().unwrap(),
        )]

        #[accont(mut)]
        pub ico_ata_for_admin: Account<'info, TokenAccount>,

        #[accont(mut)]
        pub admin: Signer<'info>,

        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub rent: Sysvar<'info, Rent>,
    }

    #[derive(Accounts)]
    pub struct DepositeIcoATA<'info> {}

    #[derive(Accounts)]
    #[instruction(_ico_ata_for_ico_program_bump: u8)]
    pub struct BuyTokens<'info>{}

    #[account]
    pub struct Data {}
}
