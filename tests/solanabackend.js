const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async () => {
  try {
    console.log("🚀 Starting test.......");

    // creating the provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Solanabackend;

    // creating an account
    const baseAccount = await anchor.web3.Keypair.generate();

    // calling startStuffOff
    let tx = await program.rpc.startStuffOff({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    console.log("📄 Transaction hash is", tx);

    // checking if there are any gifs
    let account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    console.log("🏇 GIF count:", account.totalGifs.toString());

    // adding a gif
    tx = await program.rpc.addGif("gif link", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    console.log("📄 transaction hash is", tx);
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("🏇 GIF count:", account.totalGifs.toString()); // total gifs after adding one
    console.log("🔥 Gif Objects", account.gifLinks); // prints the gif object
    console.log("❤️ Gif Likes", account.gifLinks[0].likes.toString()); // prints the likes of the gif

    // liking the gif
    await program.rpc.likeGif("gif link", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("❤️ Gif Likes", account.gifLinks[0].likes.toString()); // prints the likes of the gif
  } catch (error) {
    console.log(error);
  }
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (err) {
    console.log(err);
    process.exit(0);
  }
};

runMain();
