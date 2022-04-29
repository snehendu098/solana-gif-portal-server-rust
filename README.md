# Solana Gif Portal

Project made by Snehendu Roy. Inspired from [BuildSpace](https://buildspace.so/)

## New Code snippets: 
1. **User Likes a Gif from the gif portal**
   
   Changes in the *ItemStruct* here named as *GifHolder*
   ```
   #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
        pub struct GifHolder {
        pub gif_link: String,
        pub user_address: Pubkey,
        // Changes
        pub likes: u64,
    }
   ```
    Make a function in the `pub mod` of [Lib.rs](./programs/solanabackend/src/lib.rs)

    This function allows the user to like the gif

   ```
   // rest of your code
   pub fn like_gif(ctx: Context<Like>, gif: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // sorting
        for item in &mut base_account.gif_links {
            if item.gif_link == gif {
                item.likes += 1;
            }
        }

        Ok(())
    }
   ```

2. **Testing the like function**

   After the gif link has been created, this is the snippet which can be used to test if the like has been done or not. Check this [file](./tests/solanabackend.js) for the full code

   ```
   await program.rpc.likeGif("gif link", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("❤️ Gif Likes", account.gifLinks[0].likes.toString()); // prints the likes of the gif
   ```