
---

# 🪙 SPL Mock Staking Program

A **Solana smart contract** built with [Anchor](https://book.anchor-lang.com/) that implements a **basic staking mechanism** using mock SPL tokens.

It provides functionality for:

* Initializing a staking pool
* Depositing tokens into the pool
* Withdrawing staked tokens

---

## 📂 Project Structure

```
.
├── programs/
│   └── spl_mock/              # Anchor program written in Rust
│       ├── src/
│       │   ├── lib.rs         # Program entrypoints
│       │   ├── instructions/  # Instruction handlers
│       │   └── state/         # Account state definitions
│       └── Cargo.toml
├── tests/
│   └── spl_mock.ts            # Mocha/TypeScript tests
├── target/                    # Build artifacts (generated)
├── Anchor.toml                # Anchor configuration
├── Cargo.toml                 # Rust workspace config
└── README.md                  # Documentation
```

---

## ⚙️ Features

### 📦 State Accounts

#### 1. **Staking Pool**

```rust
pub authority: Pubkey,        // authority of the pool
pub staking_mint: Pubkey,     // mint to be staked on the pool
pub staking_vault: Pubkey,    // vault for storing staked tokens
pub total_staked: u64,        // total tokens staked
pub bump: u8,                 // bump for PDA
```

#### 2. **User Staking**

```rust
pub user: Pubkey,             // user involved in staking
pub user_token_account: Pubkey, // user's token account
pub staking_pool: Pubkey,     // reference to staking pool
pub amount_staked: u64,       // amount of tokens staked
pub bump: u8,                 // bump for PDA
```

---

### 📜 Instructions

1. **InitializeStakingPool**

   * Creates the pool PDA
   * Initializes staking mint & vault

2. **Deposit**

   * Creates a `UserStaking` PDA for the user
   * Transfers tokens from user to vault

3. **Withdraw**

   * Transfers tokens back from vault to user
   * Closes user staking account

---

## 🚀 Getting Started

### ✅ Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* [Solana CLI](https://docs.solana.com/cli/install-solana-cli)
* [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
* [Node.js](https://nodejs.org/) (for tests)

---

### ⚡ Build

```bash
anchor build
```

### 🧪 Run Tests

```bash
npm i @solana/web3.js
npm i @solana/spl-token
anchor test
```

This will:

* Deploy the program locally
* Run the provided TypeScript tests (`tests/spl_mock.ts`)
* Simulate deposit/withdraw functionality

---

## 🔬 Example Test Flow

In `tests/spl_mock.ts`, the test script performs:

1. **Airdrop SOL** to test users and mint authority
2. **Create a mint** and mint tokens to the user
3. **Transfer mint authority** to the staking pool PDA
4. Calls program instructions:

   * `initializeStakingPool`
   * `deposit(amount)`
   * `withdraw()`

---
