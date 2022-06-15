# NEAR-counter-contract
NEAR Smart Contract of Counter
[![](https://media.discordapp.net/attachments/897595033958629387/986642408521490462/unknown.png?width=1108&height=608)](https://explorer.testnet.near.org/transactions/EnVnBMP1JkcyJZGroNPjbCkDjdqM2PoxSG2VX9S3wes9)

## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```
## Deploy
You need the near-cli
```bash
npm install -g near-cli
```
And login with you near wallet
```bash
near login
```

For the deploy
```bash
near deploy --wasmFile target/wasm32-unknown-unknown/release/counter_near.wasm --accountId YOUR_ACCOUNT_HERE
```

### RUN
For using the contract in the Blockchain
```bash
near call marlos.testnet increase --accountId marlos.testnet
```
That command will execute my deployed contract, for run yours, just change the accountId.
