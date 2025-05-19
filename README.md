# Stellar Soroban dApp – Hello Counter

This project includes:
- A Soroban smart contract written in Rust
- A simple React frontend
- Example tests
- Deployment instructions

## 🧪 Prerequisites

- Rust: `rustup install stable`
- Soroban CLI: `cargo install --locked --version 0.8.2 soroban-cli`
- Node: `nvm install node`
- Yarn or npm

## 🚀 Build Contract

```bash
cd contracts/hello_counter
cargo build --target wasm32-unknown-unknown --release
```

## 🌐 Run Frontend

```bash
cd client
npm install
npm run dev
```

## 📦 Deploy Contract (Futurenet)

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_counter.wasm \
  --network futurenet
```
