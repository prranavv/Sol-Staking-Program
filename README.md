# Staking

A Solana staking program built with [Anchor](https://www.anchor-lang.com/), allowing users to stake native SOL, accumulate reward points over time, and claim SPL token rewards.

## How It Works

A user first **creates a stake account** (vault PDA), then **stakes** SOL into it. While staked, points accumulate based on the amount and duration. The user can **claim rewards** to mint SPL tokens proportional to their points, or **unstake** to withdraw SOL. A one-time **create mint** instruction initializes the reward token.

## Instructions

### Create Stake Account

Initializes the user's vault PDA with the bump seed and current timestamp.

- Vault PDA derived from `["staking", user]`
- Must be called before staking

### Stake

Deposits native SOL into the user's vault.

- **Params:** `amount` (lamports to stake)
- Recalculates accumulated points before transfer

### Unstake

Withdraws SOL from the vault back to the user.

- **Params:** `amount` (lamports to withdraw)
- Validates amount doesn't exceed vault balance

### Claim Rewards

Mints reward tokens to the user's associated token account based on accumulated points, then resets points to zero.

### Create Mint

Initializes the SPL reward token mint. Called once before any rewards can be claimed.

## Project Structure

```
├── lib.rs                  # Program entrypoint and instruction routing
├── constants.rs            # Reward rate constants
├── errors.rs               # Custom error types
├── instructions/
│   ├── mod.rs              # Module exports
│   ├── create_stake_account.rs # Vault initialization
│   ├── stake.rs            # SOL deposit into vault
│   ├── unstake.rs          # SOL withdrawal from vault
│   ├── claim_rewards.rs    # Mint reward tokens
│   └── create_mint.rs      # Initialize reward mint
└── state/
    └── vault.rs            # Vault account and point calculation
```

## Build & Test

```bash
anchor build
anchor test
```

## Program ID

```
5RqZov6TkQYw4ASzf84WPdMqSrjnc53ZfzysNHm9HDJh
```