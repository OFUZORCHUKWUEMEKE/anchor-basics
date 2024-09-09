use anchor_lang::prelude::*;

declare_id!("ARVufj6QYTBFor9gDwfva1gFETd5RG2umCty2P7a85BK");

const MIN_RATING:u8 = 1;
const MAX_RATING:u8 = 5;
const MAX_TITLE:usize=20;
const MAX_DESCRIPTION_LENGTH:usize=50;

#[program]
pub mod newanchor {
    use super::*;

    pub fn add_movie_review(
        ctx:Context<AddMovieReview>,
        title:String,
        description:String,
        rating:u8
    )->Result<()>{
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <=MAX_RATING , MovieReviewError::InvalidRating);
        // We require that the title is not longer than 20 characters
        require!(title.len() <= MAX_TITLE ,MovieReviewError::TitleTooLong);
        // We require that the description is not longer than 50 characters
        require!(title.len() <= MAX_DESCRIPTION_LENGTH ,MovieReviewError::DescriptionTooLong);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }

    pub fn update_movie_review(
        ctx:Context<UpdateMovieReview>,
        title:String,
        description:String,
        rating:u8,
    )->Result<()>{
        msg!("Movie review account space reallocated");
        msg!("Title: {}",description);
        msg!("Description: {}",description);
        msg!("Rating: {}",rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }

    pub fn delete_movie_rating(_ctx:Context<DeleteMovieReview>,title:String)->Result<()>{
        msg!("Movie review for {} deleted",title);
        Ok(())
    }

    
}

#[derive(Accounts)]
#[instruction(title:String,description:String)]
pub struct AddMovieReview<'info>{
    #[account(
        init,
        seeds =[title.as_bytes(),initializer.key().as_ref()],
        bump,
        space=DISCRIMINATOR + MovieAccountState::INIT_SPACE,
        payer=initializer
    )]
    pub movie_review:Account<'info,MovieAccountState>,
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(title:String,description:String)]
pub struct UpdateMovieReview<'info>{
    #[account(
        mut,
        seeds = [title.as_bytes()],
        bump,
        realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE,
        realloc::payer=initializer,
        realloc::zero = true,
    )]
    pub movie_review:Account<'info,MovieAccountState>,
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub system_program:Program<'info,System>,
}

// [scripts]
// test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState{
    pub reviewer:Pubkey,
    pub rating:u8,
    #[max_len(20)]
    pub title:String,
    #[max_len(50)]
    pub description:String,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteMovieReview<'info>{
    #[account(
        mut,
        seeds=[title.as_bytes(),initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub movie_review:Account<'info,MovieAccountState>,
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub system_program:Program<'info,System>
}
const DISCRIMINATOR: usize = 8;

#[error_code]
enum MovieReviewError{
    #[msg("Ratings must be betweeen 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}
