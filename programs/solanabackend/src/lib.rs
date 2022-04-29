use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solanabackend {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StrartStuff>) -> Result<()> {
        // getting reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialise the total gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = GifHolder {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            likes: 0,
        };

        // adding into the account
        base_account.gif_links.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn like_gif(ctx: Context<Like>, gif: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        for item in &mut base_account.gif_links {
            if item.gif_link == gif {
                item.likes += 1;
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StrartStuff<'info> {
    // initialises the account with init, tells payer is user, creates a 9000 byte space
    #[account(init, payer=user, space=9000)]
    // creating a base account. This is the user account
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    // TODO: KNOW
    pub user: Signer<'info>,
    // TODO: KNOW
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Like<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GifHolder {
    pub gif_link: String,
    pub user_address: Pubkey,
    // likes of a single account
    pub likes: u64,
}

#[account]
// creating a base account which holds these informations
pub struct BaseAccount {
    pub total_gifs: u64,
    // Array of all the gifs
    pub gif_links: Vec<GifHolder>,
}
