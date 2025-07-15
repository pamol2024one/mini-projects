//Import required library and packages
import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import {Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { PamolCounterPrj } from "../target/types/pamol_counter_prj";

describe("pamol-counter-prj", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.pamolCounterPrj as Program<PamolCounterPrj>;

  //Generating a new keypair for the counter account
  const counterKeypair = new Keypair();
  //Test 1
  it("Initialized Counter", async () => {
    await program.methods
      .initializeCounter()
      .accounts({
        counter: counterKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([counterKeypair])
      .rpc()
    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() == 0, "Expected initialized count to be 0")
  });
  //Test 2 - Increment
  it("Increment Counter", async () =>{
    //running the incrementCounter 1st time
    await program.methods.incrementCounter().accounts({counter: counterKeypair.publicKey}).rpc();
    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);
    assert(currentCount.count.toNumber() == 1, "Expected count to be 1");
  });

    //Test 3 - Increment Again
  it("Increment Counter Again", async () =>{
        //running the incrementCounter 2nd time
    await program.methods.incrementCounter().accounts({counter: counterKeypair.publicKey}).rpc();
    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);
    assert(currentCount.count.toNumber() == 2, "Expected count to be 2");
  });

    //Test 4 - Decrement
  it("Decrement Counter", async () =>{
    //running the decrementCounter 1st time
    await program.methods.decrementCounter().accounts({counter: counterKeypair.publicKey}).rpc();
    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);
    assert(currentCount.count.toNumber() == 1, "Expected count to be 1");
  });
});
