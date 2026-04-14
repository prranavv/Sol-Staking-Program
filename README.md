<h1 align="center">Staking</h1>
<p align="center"><strong>Native SOL Staking with SPL Token Rewards on Solana</strong></p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75+-DEA584?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/Solana-2.0-9945FF?style=flat-square&logo=solana" />
  <img src="https://img.shields.io/badge/Anchor-1.0-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/SPL--Token-✓-00D18C?style=flat-square" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" />
</p>

<p align="center">
  A Solana staking program built with Anchor that allows users to stake native SOL, accumulate reward points over time based on amount and duration, and claim SPL token rewards. Stake more, stake longer, earn more.
</p>

---

## The Problem

Staking mechanisms need to fairly reward users based on both how much they stake and how long they keep it staked. A simple balance check at claim time doesn't capture duration — someone who staked 10 SOL for a month should earn more than someone who staked 10 SOL for a minute. The reward calculation needs to be continuous and tamper-proof.

## The Solution

This program tracks **reward points** that accumulate proportionally to `amount × time`. Every time the vault state changes (stake, unstake, or claim), points are recalculated based on the current balance and elapsed time since the last update. When the user claims rewards, SPL tokens are minted proportional to their accumulated points, and the counter resets to zero.

**Program ID:** `5RqZov6TkQYw4ASzf84WPdMqSrjnc53ZfzysNHm9HDJh`

---

## How It Works

```
┌──────────────────────────────────────────────────────────────┐
│                    1. Create Stake Account                   │
│                                                              │
│   User ──── create_stake_account ────►  Vault PDA            │
│              one-time setup             seeds: [staking,user]│
│                                         stores: bump, ts     │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                        2. Stake SOL                          │
│                                                              │
│   User ──── SOL (lamports) ────►  Vault PDA                  │
│              system transfer        balance increases         │
│                                     points recalculated       │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                     3. Accumulate Points                     │
│                                                              │
│   points += vault_balance × (now - last_update)              │
│                                                              │
│   Recalculated on every stake, unstake, and claim action     │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                     4a. Claim Rewards                        │
│                                                              │
│   Vault ──── points ────►  Reward Mint ────►  User ATA       │
│              read points    mint SPL tokens    receive tokens │
│              reset to 0     proportional       reward earned  │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                      4b. Unstake SOL                         │
│                                                              │
│   Vault ──── SOL (lamports) ────►  User                      │
│              withdraw               points recalculated      │
│              validates balance       balance decreases        │
└──────────────────────────────────────────────────────────────┘
```

---

## Features

- **Native SOL staking** — deposit and withdraw SOL directly into a program-controlled vault PDA
- **Time-weighted rewards** — points accumulate based on `balance × duration`, rewarding both size and commitment
- **SPL token rewards** — claim mints reward tokens proportional to accumulated points, then resets the counter
- **Continuous accrual** — points are recalculated on every state change (stake, unstake, claim), not on a fixed schedule
- **Per-user vaults** — each user gets their own vault PDA derived from `["staking", user]`
- **One-time mint setup** — a dedicated `create_mint` instruction initializes the reward token before any claims

---

## Instructions

| Instruction | Parameters | Description |
|---|---|---|
| **`create_stake_account`** | — | Initializes the user's vault PDA with the bump seed and current timestamp. Must be called before staking. Vault derived from `["staking", user]`. |
| **`stake`** | `amount: u64` | Deposits native SOL (in lamports) into the vault. Recalculates accumulated points before transfer. |
| **`unstake`** | `amount: u64` | Withdraws SOL from the vault back to the user. Validates that the amount doesn't exceed the vault balance. Recalculates points before transfer. |
| **`claim_rewards`** | — | Mints SPL reward tokens to the user's associated token account based on accumulated points, then resets points to zero. |
| **`create_mint`** | — | Initializes the SPL reward token mint. Called once before any rewards can be claimed. |

---

## Account Structure

### Vault (PDA)

| Field | Type | Description |
|---|---|---|
| `bump` | `u8` | PDA bump seed |
| `points` | `u64` | Accumulated reward points |
| `last_updated` | `i64` | Timestamp of last state change |

### PDA Seeds

| Account | Seeds |
|---|---|
| **Vault** | `["staking", user.key()]` |

---

## Project Structure

```
staking/
├── src/
│   ├── lib.rs                      # Program entrypoint and instruction routing
│   ├── constants.rs                # Reward rate constants
│   ├── errors.rs                   # Custom error types
│   ├── instructions/
│   │   ├── mod.rs                  # Module exports
│   │   ├── create_stake_account.rs # Vault initialization
│   │   ├── stake.rs                # SOL deposit into vault
│   │   ├── unstake.rs              # SOL withdrawal from vault
│   │   ├── claim_rewards.rs        # Mint reward tokens
│   │   └── create_mint.rs          # Initialize reward mint
│   └── state/
│       └── vault.rs                # Vault account and point calculation
├── Cargo.toml
└── README.md
```

---

## Quick Start

### Prerequisites

- Rust 1.75+
- Solana CLI 2.0+
- Anchor CLI 1.0+

### 1. Clone and build

```bash
git clone https://github.com/prranavv/Sol-Staking-Progra.git
cd Sol-Staking-Progra

anchor build
```

### 2. Deploy

```bash
anchor deploy
```

### 3. Test

```bash
anchor test
```

---

## Tech Stack

| Component | Technology | Purpose |
|---|---|---|
| **Runtime** | Solana | High-throughput L1 blockchain |
| **Framework** | Anchor 1.0 | Solana program development framework with IDL generation |
| **Token Standard** | SPL Token | Reward token minting and distribution |
| **Language** | Rust | On-chain program logic |

---

## FAQ's

**"How are reward points calculated?"**
> Every time the vault state changes (stake, unstake, or claim), the program calculates `elapsed_time = now - last_updated` and adds `vault_balance × elapsed_time` to the accumulated points. This means rewards scale linearly with both the amount staked and the duration.

**"Why recalculate points on every action instead of at claim time?"**
> If points were only calculated at claim time, a user could stake 1 SOL for a month, then stake 1000 SOL one second before claiming, and get rewarded as if they had 1001 SOL staked the entire time. Recalculating on every state change snapshots the balance, preventing this manipulation.

**"Why a separate `create_mint` instruction?"**
> The reward token mint only needs to be created once for the entire program. Separating it from the staking flow keeps the instructions focused and avoids paying mint creation rent on every new stake account.

**"Why a separate `create_stake_account` instruction?"**
> The vault PDA needs to exist before any SOL can be deposited into it. Splitting account creation from staking makes the instructions composable — you create the account once and stake/unstake as many times as you want.

**"What token do users receive as rewards?"**
> A custom SPL token whose mint is controlled by the program. The program is the mint authority, so only the `claim_rewards` instruction can mint new tokens. The token has no intrinsic value — it represents the reward mechanism itself.

---

## Disclaimer

This staking program is a learning and portfolio project demonstrating Solana program development with the Anchor framework. It has not been audited and is not intended for production use with real funds. The reward token has no real-world value. Always conduct a thorough security audit before deploying any program that handles user assets.

---

## License

MIT — see [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Built by <a href="https://github.com/prranavv">prranavv</a></sub>
</p>
