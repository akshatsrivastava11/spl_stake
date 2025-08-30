
# SPL Mock Staking Program

This project is a **Solana smart contract** (built with [Anchor](https://book.anchor-lang.com/)) that implements a **basic staking mechanism** using mock SPL tokens.
It provides functionality for:

* Initializing a staking pool
* Depositing tokens into the pool
* Withdrawing staked tokens

---

## 📂 Project Structure

```
.
├── programs/
│   └── spl_mock/           # Anchor program written in Rust
│       ├── src/
│       │   ├── lib.rs      # Program entrypoints
│       │   ├── instructions/ # Instruction handlers
│       │   └── state/      # Account state definitions
│       └── Cargo.toml
├── tests/
│   └── spl_mock.ts         # Mocha/TypeScript tests
├── target/                 # Build artifacts (generated)
├── Anchor.toml             # Anchor configuration
├── Cargo.toml              # Rust workspace config
└── README.md               # Documentation
```

---

## ⚙️ Features

* **StakingPool account** – Stores global staking pool state:

  * Authority
  * Staking mint
  * Vault account
  * Total staked tokens
* **UserStaking account** – Tracks each user’s staked amount and token account.

### Instructions

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

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* [Solana CLI](https://docs.solana.com/cli/install-solana-cli)
* [Anchor](https://book.anchor-lang.com/getting_started/installation.html)
* [Node.js](https://nodejs.org/) (for tests)

---

### Build

```bash
anchor build
```

### Run Tests

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

## 🧪 Example Test Flow

In `tests/spl_mock.ts`, the test script does:

1. **Airdrops SOL** to the test users and mint authority
2. **Creates a mint** and mints tokens to the user
3. **Transfers mint authority** to the staking pool PDA
4. Calls:

   * `initializeStakingPool`
   * `deposit(amount)`
   * `withdraw()`

---

