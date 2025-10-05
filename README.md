# MultiversX Token Staking Smart Contract

A comprehensive smart contract that enables users to stake tokens in a single farm and earn time-based rewards on the MultiversX blockchain.

## Overview

This smart contract implements a token staking mechanism where users can:
- Stake tokens in a farm and receive share tokens (NFTs) representing their stake
- Earn rewards over time based on configurable reward rates
- Claim accumulated rewards or have them auto-claimed during operations
- Unstake tokens by burning share tokens to retrieve original tokens plus rewards
- Benefit from automatic share token merging to reduce NFT clutter

## Key Features

### 🏭 **Farm Management**
- Create and configure a single staking farm with custom parameters
- Set staking tokens, reward tokens, and reward rates
- Define farm duration with start/end timestamps
- Manage rewards reserve funding

### 🥩 **Staking Operations**
- Stake tokens and receive share tokens (NFTs) as proof of stake
- Automatic reward calculation based on staking duration
- Share token merging functionality to consolidate multiple stakes
- Unstaking with automatic reward claiming

### 💰 **Reward System**
- Time-based reward accrual using configurable rates
- Automatic reward distribution during staking/unstaking operations
- Reward per share calculation for fair distribution
- Flexible reward token support (EGLD or ESDT)

### 🛡️ **Security & Administration**
- Multi-admin management system
- Pause/unpause functionality for emergency situations
- Role-based access control
- Secure reward reserve management

## Project Structure

```
src/
├── lib.rs              # Main contract trait and initialization
├── admins.rs           # Admin management module
├── farm.rs             # Farm operations and management
├── staking.rs          # Staking and unstaking functionality
├── rewards.rs          # Reward calculation and claiming
└── pause.rs            # Pause mechanism

common/
├── constants/          # Shared constants and configuration
├── errors/             # Error definitions and messages
└── structs/            # Core data structures (Farm, ShareToken, etc.)

interaction/            # Deployment and interaction scripts
meta/                   # Contract metadata and configuration
tests/                  # Unit and integration tests
wasm/                   # WebAssembly build configuration
```

## How It Works

1. **Farm Setup**: Contract owner creates a single farm with staking/reward token pairs and reward parameters
2. **Token Staking**: Users stake tokens and receive share tokens (NFTs) representing their stake position
3. **Reward Accrual**: Rewards accumulate continuously based on stake amount, duration, and farm reward rate
4. **Share Token Management**: Multiple share tokens can be merged automatically to reduce NFT clutter
5. **Reward Claiming**: Users can claim rewards manually or automatically during staking/unstaking operations
6. **Unstaking**: Users burn share tokens to retrieve original staked tokens plus accumulated rewards

## Multiple Farms

This contract is designed to manage a single farm per deployment. For multiple farms, deploy this contract multiple times as a template, with each deployment configured for a specific staking/reward token pair.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.