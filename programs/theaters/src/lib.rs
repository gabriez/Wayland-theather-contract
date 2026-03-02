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

    pub fn add_show(ctx: Context<ModifyTheather>, title: String, date: String) -> Result<()> {
        require!(title.len() <= 50, TheatherError::TitleTooLong);
        require!(date.len() <= 30, TheatherError::DateTooLong);
        require!(
            theather.upcoming_shows.len() < 90,
            TheatherError::MaxShowsReached
        );

        let index: u8;

        let mut theather = ctx.accounts.theather;

        if !theather.cleared_index.is_empty() {
            index = theather.cleared_index[0];
            theather.cleared_index.remove(0);
        } else {
            index = theather.upcoming_shows.len() as u8;
        }

        let show = Shows {
            id: index,
            title,
            date,
        };

        theather.upcoming_shows.push(show);

        Ok(())
    }

    pub fn delete_show(ctx: Context<ModifyTheather>, id: u8) -> Result<()> {
        let mut theather = ctx.accounts.theather;
        let show_index = theather
            .upcoming_shows
            .iter()
            .position(|show| show.id == id)
            .ok_or(TheathersError::ShowNotFound)?;

        theather.upcoming_shows.swap_remove(index);
        theather.cleared_index.push(id);

        Ok(())
    }

    pub fn update_show(
        ctx: Context<ModifyTheather>,
        id: u8,
        title: Option<String>,
        date: Option<String>,
    ) -> Result<()> {
        let mut theather = ctx.accounts.theather;
        let show_index = theather
            .upcoming_shows
            .iter()
            .position(|show| show.id == id)
            .ok_or(TheathersError::ShowNotFound)?;

        let show = &mut theather.upcoming_shows[show_index];

        if let Some(title) = title {
            require!(title.len() <= 50, TheatherError::TitleTooLong);
            show.title = title;
        }

        if let Some(date) = date {
            require!(date.len() <= 30, TheatherError::DateTooLong);
            show.date = date;
        }

        Ok(())
    }

    pub fn print_shows(ctx: Context<ModifyTheather>) -> Result<()> {
        let mut theather = ctx.accounts.theather;
        msg!("Upcoming shows: {:?}", theather.upcoming_shows);
        Ok(())
    }

    pub fn print_show(ctx: Context<ModifyTheather>, id: u8) -> Result<()> {
        let mut theather = ctx.accounts.theather;
        let show_index = theather
            .upcoming_shows
            .iter()
            .position(|show| show.id == id)
            .ok_or(TheathersError::ShowNotFound)?;

        let show = theather.upcoming_shows[show_index];

        msg!(
            "Show id: {}, title: {}, date: {}",
            show.id,
            show.title,
            show.date
        );

        Ok(())
    }
}

#[error_code]
pub enum TheatherError {
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
    #[max_len(255)]
    cleared_index: Vec<u8>,
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
