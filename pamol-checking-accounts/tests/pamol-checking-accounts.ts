import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PamolCheckingAccounts } from "../target/types/pamol_checking_accounts";
import { Keypair, SystemProgram, Transaction , sendAndConfirmTransaction} from "@solana/web3.js";

describe("pamol-checking-accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.pamolCheckingAccounts as Program<PamolCheckingAccounts>;

  //We will create it ahead of time so that our program will try to modify it
  const accountToChange = new Keypair();
  //Our program will create this
  const accountToCreate = new Keypair();


  it("Create an account owned by our program", async () => {
    //Creating instruction to create an account
    const instruction = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space:0,
      programId: program.programId, //Our program
    });
    //Creating Transanction by adding all the created instructions
    const transaction = new Transaction().add(instruction);
    //Send to the Transanction to blockchain to process and confirm
    await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer, accountToChange]);
  });
  it("Check accounts", async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer:wallet.publicKey,
        accountToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
      })
      .rpc();
  });


});
