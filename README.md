# Solana Ticketing

An Anchor-based Solana event ticketing program.

## Features

- Create events with capacity, schedule, venue, and ticket price
- Sell PDA-backed tickets
- Check tickets in during an event
- Prevent duplicate check-ins
- Cancel events before they start

## Requirements

- Rust and Solana CLI
- Anchor CLI
- Node.js and npm

## Setup

```bash
npm install
cp .env.example .env
```

The `.env` file is local-only. Never commit a private key or wallet JSON file.

## Test

Run the local integration suite:

```bash
anchor test
```

The tests build the program, start a local validator, deploy the program, and test event creation, ticket purchase, check-in, replay protection, and cancellation.

## Deploy

Create or use a deployment wallet:

```bash
solana-keygen new --outfile ~/.config/solana/deployer.json
solana config set --keypair ~/.config/solana/deployer.json
```

Deploy to Solana devnet:

```bash
solana airdrop 2 --url devnet
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \\
ANCHOR_WALLET=~/.config/solana/deployer.json \\
anchor deploy
```

The wallet path points to a local Solana keypair JSON file. Do not place the private key directly in `.env` or source code.

## Project Structure

```text
programs/ticketing/src/       Anchor program
programs/ticketing/src/state.rs
programs/ticketing/src/instructions/
tests/ticketing.ts             Integration tests
Anchor.toml                   Anchor configuration
.env.example                  Environment template
```

The deployed program ID is configured in `Anchor.toml`:

```text
GQFxmaiXc9sbyzrAtC9oTqdxbB9D6YZvTCzxxHkKefuZ
```
