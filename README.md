# DeFi Yield Optimizer

A decentralized yield optimization protocol built on NEAR Protocol that automatically maximizes users' yield returns across various DeFi platforms.

## Features

- Multiple yield farming strategies
- Automated reward claiming
- TVL tracking
- User position management
- Dynamic APY updates
- Governance token integration

## Prerequisites

- [NEAR CLI](https://docs.near.org/tools/near-cli#setup)
- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-near](https://github.com/near/cargo-near)

## Building and Deployment

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Deploy the contract
near deploy --accountId YOUR_ACCOUNT_ID --wasmFile target/wasm32-unknown-unknown/release/defi_yield_optimizer.wasm
```

## Contract Initialization

Initialize the contract with an owner account and governance token:

```bash
near call YOUR_CONTRACT_ID new '{"owner_id": "OWNER_ACCOUNT_ID", "governance_token": "TOKEN_ACCOUNT_ID"}' --accountId YOUR_ACCOUNT_ID
```

## Interacting with the Contract

### Strategy Management

#### Add a New Strategy
```bash
near call YOUR_CONTRACT_ID add_strategy '{
    "name": "Farming Strategy",
    "protocol": "Protocol Name",
    "apy": 1000,
    "min_deposit": "1000000000000000000000000"
}' --accountId OWNER_ACCOUNT_ID --deposit 1
```
Note: APY is in basis points (1000 = 10%)

#### Update Strategy APY
```bash
near call YOUR_CONTRACT_ID update_strategy_apy '{
    "strategy_id": 0,
    "new_apy": 1200
}' --accountId OWNER_ACCOUNT_ID
```

### User Operations

#### Deposit into a Strategy
```bash
near call YOUR_CONTRACT_ID deposit '{
    "strategy_id": 0
}' --accountId YOUR_ACCOUNT_ID --deposit 5
```
Note: Deposit amount is in yoctoNEAR (1 NEAR = 1e24 yoctoNEAR)

#### Claim Rewards
```bash
near call YOUR_CONTRACT_ID claim_rewards '{
    "position_index": 0
}' --accountId YOUR_ACCOUNT_ID
```

### View Methods

#### Get Strategy Details
```bash
near view YOUR_CONTRACT_ID get_strategy '{"strategy_id": 0}'
```

#### Get User Positions
```bash
near view YOUR_CONTRACT_ID get_user_positions '{"user_id": "USER_ACCOUNT_ID"}'
```

#### Get Total TVL
```bash
near view YOUR_CONTRACT_ID get_total_tvl '{}'
```

## Response Types

### Strategy
```typescript
{
    name: string,
    protocol: string,
    apy: number,        // in basis points (1/100th of a percent)
    tvl: string,        // in yoctoNEAR
    min_deposit: string, // in yoctoNEAR
    is_active: boolean,
    last_update: number
}
```

### UserPosition
```typescript
{
    amount: string,           // in yoctoNEAR
    strategy_id: number,
    rewards_claimed: string,  // in yoctoNEAR
    deposit_timestamp: number
}
```

## Important Notes

1. All monetary values are in yoctoNEAR (1 NEAR = 10^24 yoctoNEAR)
2. APY values are in basis points (1 basis point = 0.01%)
3. The minimum deposit amount is set to 1 NEAR
4. Only the contract owner can add strategies and update APYs
5. Users need to have sufficient NEAR balance for deposits and gas fees

## Security Considerations

- Always verify transaction parameters before signing
- Ensure you're interacting with the correct contract address
- Keep your private keys secure
- Review strategy parameters before depositing

## Testing

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
