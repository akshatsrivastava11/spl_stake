import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplMock } from "../target/types/spl_mock";
import { LAMPORTS_PER_SOL, PublicKey, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import {createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID, setAuthority, AuthorityType} from '@solana/spl-token'
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("spl_mock", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.splMock as Program<SplMock>;
  const user = anchor.web3.Keypair.generate();
  const authority = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate(); // New keypair for mint authority
  let staking_mint: PublicKey;
  let staking_vault: PublicKey;
  let staking_pool: PublicKey;
  let user_token_account: PublicKey;
  let userStaking: PublicKey;

  before(async () => {
    // Airdrop the signer and authority of the staking pool
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 2 * LAMPORTS_PER_SOL)
    )
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(authority.publicKey, 2 * LAMPORTS_PER_SOL)
    )
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(mintAuthority.publicKey, 2 * LAMPORTS_PER_SOL)
    )

    // PDA for staking pool
    staking_pool = PublicKey.findProgramAddressSync(
      [Buffer.from("staking_pool")],
      program.programId
    )[0]

    // PDA for user staking
    userStaking = PublicKey.findProgramAddressSync(
      [Buffer.from("user_staking"), user.publicKey.toBuffer()],
      program.programId
    )[0]

    // Create mint with mintAuthority initially
    staking_mint = await createMint(
      provider.connection, 
      mintAuthority, 
      mintAuthority.publicKey,
      null, 
      6
    );
    
    staking_vault = await getAssociatedTokenAddress(staking_mint, staking_pool, true);
    user_token_account = (await getOrCreateAssociatedTokenAccount(
      provider.connection, 
      user, 
      staking_mint, 
      user.publicKey, 
      false
    )).address;

    // Mint tokens to user account first (while we still control the mint)
    await mintTo(
      provider.connection,
      mintAuthority,
      staking_mint,
      user_token_account,
      mintAuthority.publicKey,
      1000000
    )

    // Transfer mint authority to the staking_pool PDA
    await setAuthority(
      provider.connection,
      mintAuthority, // Current authority (signer)
      staking_mint,
      mintAuthority.publicKey, // Current authority
      AuthorityType.MintTokens,
      staking_pool // New authority (PDA)
    );
  })

  it("Initialize_staking_pool", async () => {
    await program.methods.initializeStakingPool().accounts({
      signer: authority.publicKey,
      stakingMint: staking_mint,
      stakingVault: staking_vault,
      stakingPool: staking_pool,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SYSTEM_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY
    }).signers([authority]).rpc()
  })

  it("Deposit", async () => {
    await program.methods.deposit(new BN(100)).accounts({
      signer: user.publicKey,
      stakingPool: staking_pool,
      stakingVault: staking_vault,
      stakingMint: staking_mint,
      userStaking: userStaking,
      userTokenAccount: user_token_account,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID
    }).signers([user]).rpc()
  })
  it("Withdraw",async()=>{
    await program.methods.withdraw().accounts({
         signer: user.publicKey,
      stakingPool: staking_pool,
      stakingVault: staking_vault,
      stakingMint: staking_mint,
      userStaking: userStaking,
      userTokenAccount: user_token_account,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID
    }).signers([user]).rpc()
  })
});