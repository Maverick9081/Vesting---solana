import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { Vesting } from "../target/types/vesting";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  Connection,
  Commitment,
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import * as Token from "@solana/spl-token";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { min } from "bn.js";

describe("Vesting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.Vesting as Program<Vesting>;



  let mint = null ;
  let ownerTokenAccount = null;

  const amount = 1000000 * 10 ** 9;

  const owner = (provider.wallet as NodeWallet).payer;
  const beneficiary = anchor.web3.Keypair.generate();
  const mintAuthority = owner;

  it('Initialize vesting account', async () => {
    mint = await Token.createMint(
      provider.connection,
      owner,
      mintAuthority.publicKey,
      null,
      9,
    );
    console.log(mint)


    ownerTokenAccount = await Token.getAssociatedTokenAddress(
      mint,
      owner.publicKey
    );


    console.log(await Token.mintTo(
      provider.connection,
      owner,
      mint,
      ownerTokenAccount,
      owner,
      10000000,
    ));

    

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.claim().rpc();
    console.log("Your transaction signature", tx);
  });
});
});
