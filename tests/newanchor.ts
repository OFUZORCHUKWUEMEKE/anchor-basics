import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {expect} from "chai";
import { Newanchor } from "../target/types/newanchor";

describe("newanchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.Newanchor as Program<Newanchor>;

  const movie ={
    title:"Just a test Movie",
    description:"Wow what a good movie it was real great",
    rating:5
  }

  const [moviePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(movie.title),provider.wallet.publicKey.toBuffer()],
    program.programId
  )
  

  
  it("Movie review is added`", async () => {
    // const tx =await program.methods.add_movie_review(movie.title,movie.description,movie.rating).rpc();
    const tx = await program.methods.addMovieReview(movie.title,movie.description,movie.rating).rpc();

    // const account = await program.account.MovieAccountState.fetch(moviePda);
    const account = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title===account.title);
    expect(movie.rating === account.rating);
    expect(movie.description === account.description);
    expect(account.reviewer === provider.wallet.publicKey);
  });

 
  it("Movie review is updated`", async () => {
    const newDescription = "Wow this is new";
    const newRating = 4;

    // const tx = await program.methods.update_movie_review(movie.title,newDescription,newRating).rpc();
    const tx = await program.methods.updateMovieReview(movie.title,newDescription,newRating).rpc();
    const account = await program.account.movieAccountState.fetch(moviePda);
    expect(movie.title === account.title);
    expect(newRating === account.rating);
    expect(newDescription === account.description);
    expect(account.reviewer === provider.wallet.publicKey);
  });
 
  it("Deletes a movie review", async () => {
    // const tx = await program.methods.deleteMovieReview(movie.title).rpc();
    const tx = await program.methods.deleteMovieRating(movie.title).rpc();
  });

  it("Is initialized!", async () => {
    // // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature");
  });
});

// Wrote new keypair to /root/.config/solana/id.json
// ===================================================================================
// pubkey: 7xH6QfhCj1arfC42ZUfiYzyhmyCmKhZqjyGu8bzKXvYB
// ===================================================================================
// Save this seed phrase and your BIP39 passphrase to recover your new keypair:
// brisk heavy thought quote wasp apology kangaroo wealth panic harbor mushroom caught

// [2024-09-05T16:25:42.719543488Z ERROR cargo_build_sbf] Failed to install platform-tools: Unable to write "/root/.cache/solana/v1.41/platform-tools/tmp-platform-tools-linux-x86_64.tar.bz2": Custom { kind: Other, error: reqwest::Error { kind: Decode, source: TimedOut } }

// rm -rf ~/.cache/solana

// source ~/.bashrc
// solana --version