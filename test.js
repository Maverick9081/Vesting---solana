
import * as anchor from "@project-serum/anchor";
import pkgg from "@project-serum/anchor"
const  {BN} = pkgg; 
import pkg from "@solana/web3.js"
import { Buffer } from "buffer";
const { Connection,Keypair,LogsFilter,clusterApiUrl, PublicKey,Transaction,TransactionInstruction,sendAndConfirmTransaction,web3} = pkg;
import fs from "fs"
import { getAssociatedTokenAddress,getOrCreateAssociatedTokenAccount, initializeAccountInstructionData} from "@solana/spl-token";
import { Console } from "console";
import { runInContext } from "vm";

async function run(){

  const secretKey = fs.readFileSync(
    "/home/Abhi/hii.json",
    "utf8"
  );
  const keypair = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(secretKey))
  );
 
  const walletWrapper = new anchor.Wallet(keypair);

  const secretKey2 = fs.readFileSync(
    "./hi.json",
    "utf8"
  );

  const keypair2 = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(secretKey2))
  );
 
  const walletWrapper2 = new anchor.Wallet(keypair2);


  const contract = new PublicKey("AFLwi1VLdGgtHYmxdg2EeqkYvv2oMWwJE4FpTbQfroL1")
  
  const commitment = 'processed';
  const connection = new anchor.web3.Connection('https://api.devnet.solana.com');
  const options = anchor.AnchorProvider.defaultOptions();
  const provider = new anchor.AnchorProvider(connection, walletWrapper2, options);
  const idl = await anchor.Program.fetchIdl(
    contract,
    provider,
  );
  // console.log(idl)

  const program =new anchor.Program(
    idl,
    contract,
    provider,
  );

  const mint = new PublicKey("4BbGWTVHvvBdiawyoqMPf39jnrmYStHNC51uobR7eGNK");

  const owner = walletWrapper.publicKey;
  const ownerAta = await getAssociatedTokenAddress(mint,owner);
  const beneficiary = walletWrapper2.publicKey

  const beneficiaryAta = await getOrCreateAssociatedTokenAccount(connection,walletWrapper2.payer,mint,beneficiary);

  console.log(beneficiaryAta.address)
  

  const vaultAccount = await PublicKey.findProgramAddress([
    Buffer.from('vault'),
    beneficiary.toBuffer()
  ],
    contract);

    console.log("vault account is",vaultAccount[0].toBase58())
  const vestingAccount = await PublicKey.findProgramAddress([
    Buffer.from('vesting'),
    beneficiary.toBuffer()
  ],contract);

  const vaultAuthority = await PublicKey.findProgramAddress([
    Buffer.from('vault')
  ],contract);


  console.log("vesting account is",vestingAccount[0].toBase58())

  const clock = new PublicKey("SysvarC1ock11111111111111111111111111111111")

  const rent = new PublicKey("SysvarRent111111111111111111111111111111111");

  const tokenProgram = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

  const systemProgram = new PublicKey("11111111111111111111111111111111");

  const amount = new BN(10000)

  const cliff = new BN(1)

  const startTime = new BN(0)

  const endTime = new BN(5000)

  const per = new BN(7)

  

  // const tx = await program.methods.addBeneficiary(amount,cliff,startTime,endTime,per).accounts({
  //   owner : owner,
  //   ownerAta : ownerAta,
  //   beneficiary : beneficiary,
  //   vaultAccount : vaultAccount[0],
  //   mint : mint,
  //   vestingAccount : vestingAccount[0],
  //   clock : clock,
  //   tokenProgram : tokenProgram,
  //   systemProgram : systemProgram,
  //   rent : rent
  // }).rpc()

  const tx = await program.methods.claim().accounts({
    beneficiary : beneficiary,
    beneficiaryAta : beneficiaryAta.address,
    vaultAccount : vaultAccount[0],
    vestingAccount : vestingAccount[0],
    vaultAuthority : vaultAuthority[0],
    clock : clock,
    tokenProgram : tokenProgram,
    systemProgram : systemProgram,
    rent : rent
  }).rpc()

  console.log(tx);
}
run();