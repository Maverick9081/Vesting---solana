import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { Vesting } from "../target/types/vesting";

import * as Token from "@solana/spl-token";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';


describe("Vesting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.Vesting as Program<Vesting>;



  let mint = null ;
  let ownerTokenAccount = null;

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
    // console.log(mint)


    ownerTokenAccount = await Token.getOrCreateAssociatedTokenAccount(
      provider.connection,
      owner,
      mint,
      owner.publicKey
    );

  //  let  beneficiaryTokenAccount = await Token.getOrCreateAssociatedTokenAccount(
  //     provider.connection,
  //     beneficiary,
  //     mint,
  //     beneficiary.publicKey
  //   );


    // console.log("owner",ownerTokenAccount);


   await Token.mintTo(
      provider.connection,
      owner,
      mint,
      ownerTokenAccount.address,
      owner,
      10000000,
    );

    const vaultAccount = await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from('vault'),
      beneficiary.publicKey.toBuffer()
    ],
      program.programId);
  
      // console.log("vault account is",vaultAccount[0].toBase58())
    const vestingAccount = await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from('vesting'),
      beneficiary.publicKey.toBuffer()
    ],program.programId);
  
    const vaultAuthority = await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from('vault')
    ],program.programId);

    const clock = new anchor.web3.PublicKey("SysvarC1ock11111111111111111111111111111111")

  const rent = new anchor.web3.PublicKey("SysvarRent111111111111111111111111111111111");

  const tokenProgram = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

  const systemProgram = new anchor.web3.PublicKey("11111111111111111111111111111111");

  

    const amount = new anchor.BN(10000)

  const cliff = new anchor.BN(1)

  const startTime = new anchor.BN(0)

  const endTime = new anchor.BN(5000)

  const per = new anchor.BN(7)

  const tx = await program.methods.addBeneficiary(amount,cliff,startTime,endTime,per).accounts({
    owner : owner.publicKey,
    ownerAta : ownerTokenAccount.address,
    beneficiary : beneficiary.publicKey,
    vaultAccount : vaultAccount[0],
    mint : mint,
    vestingAccount : vestingAccount[0],
    clock : clock,
    tokenProgram : tokenProgram,
    systemProgram : systemProgram,
    rent : rent
  }).rpc()
  console.log("Your transaction signature", tx);

  let  beneficiaryTokenAccount = await Token.getOrCreateAssociatedTokenAccount(
    provider.connection,
    owner,
    mint,
    beneficiary.publicKey
  );



    const tx2 = await program.methods.claim().accounts({
    beneficiary : beneficiary.publicKey,
    beneficiaryAta : beneficiaryTokenAccount.address,
    vaultAccount : vaultAccount[0],
    vestingAccount : vestingAccount[0],
    vaultAuthority : vaultAuthority[0],
    clock : clock,
    tokenProgram : tokenProgram,
    systemProgram : systemProgram,
    rent : rent
  }).signers([beneficiary]).rpc()

  console.log(tx2)

  it("Is initialized!", async () => {
    // Add your test here.
   
  });
});
});
