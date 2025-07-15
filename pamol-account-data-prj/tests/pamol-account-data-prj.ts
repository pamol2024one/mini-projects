import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from '@solana/web3.js';
import { PamolAccountDataPrj } from "../target/types/pamol_account_data_prj";

describe("pamol-account-data-prj", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.pamolAccountDataPrj as Program<PamolAccountDataPrj>;

  // Generate a new keypair for the addressInfo account
  const addressInfoAccount = new Keypair();
  
  //Create the account address data
  it('Create the address info account', async () => {
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    // Instruction Ix data
    const addressInfo = {
      name: 'Joe C',
      houseNumber: 136,
      street: 'Mile High Dr.',
      city: 'Solana Beach',
    };

    await program.methods
      .createAddressInfo(addressInfo.name, addressInfo.houseNumber, addressInfo.street, addressInfo.city)
      .accounts({
        addressInfo: addressInfoAccount.publicKey,
        authority: payer.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();
  });

  //Read the account info  
  it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });
});
