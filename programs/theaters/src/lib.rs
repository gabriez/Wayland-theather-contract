use anchor_lang::prelude::*;
use std::collections::HashMap;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("7pT9bkPrxz48GnEqmUKxWCyMAFVtS6ksfMaYRNajs3Wd");

#[program]
mod theaters {
    use super::*;
    pub fn initialize(ctx: Context<CreateTheather>) -> Result<()> {
        Ok(())
    }

    pub fn add_show(ctx: Context<Initialize>, title: String, date: String) -> Result<()> {
        require!(title.len() <= 50, TheatherError::TitleTooLong);
        require!(date.len() <= 30, TheatherError::DateTooLong);

        Ok(())
    }

    pub fn delete_show(ctx: Context<Initialize>, id: u8) -> Result<()> {
        Ok(())
    }

    pub fn update_show(ctx: Context<Initialize>, id: u8) -> Result<()> {
        Ok(())
    }

    pub fn print_shows(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn print_show(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[error_code]
pub enum TheathersError {
    #[msg("Title should be less or equal to 50 characters")]
    TitleTooLong,
    #[msg("Date should be less or equal to 30 characters and include hours")]
    DateTooLong,
    #[msg("Theather can only have 90 upcoming shows")]
    MaxShowsReached,
    #[msg("Show with the provided id was not found")]
    ShowNotFound,
}

/// Represents the data stored in a PDA account. Upcoming shows has a maximum len of 90 shows
#[account]
#[derive(InitSpace)]
pub struct Theather {
    owner: Pubkey,
    #[max_len(90)]
    upcoming_shows: Vec<Shows>,
}

/// Struct that represent shows we are storing on the theathers PDA account
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Shows {
    id: u8,

    #[max_len(50)]
    title: String,
    // It should include hours
    #[max_len(30)]
    date: String,
}

/// Creates PDA account called theather
/// The initializer of the account should always sign the transaction so it is possible to create the
/// PDA
#[derive(Accounts)]
pub struct CreateTheather<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        space = 8 + Theather::INIT_SPACE,
        seeds = [b"theather", initializer.key().as_ref()],
        bump
    )]
    pub theather: Account<'info, Theather>,
    pub system_program: Account<'info, Theather>,
}

#[derive(Accounts)]
pub struct ModifyTheather<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"theather", initializer.key().as_ref()],
        bump
    )]
    pub theather: Account<'info, Theather>,
}
