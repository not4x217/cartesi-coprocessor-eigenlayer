Demonstrates querying on-chain state (Eigenlayer) "from within" cartesi virtual machine.

## Running demo

1. Run coprocessor devnet

```bash
git clone https://github.com/zippiehq/cartesi-coprocessor -b use-local-operator-image
cd cartesi-coprocessor
docker compose -f docker-compose-devnet.yaml up
```

2. Build operator node conatiner

```bash
git clone https://github.com/zippiehq/cartesi-coprocessor-operator -b fix-gio
cd cartesi-coprocessor-operator
docker build -t cartesi-coprocessor-operator .
```

3. Publish application code

```bash
cd cartesi-coprocessor-eigenlayer
cartesi-coprocessor publish --network devnet
```

Note down machine hash for the next step.

4. Deploy smart contract

```bash
cd cartesi-coprocessor-eigenlayer/contracts
forge create --broadcast \
  --rpc-url http://127.0.0.1:8545 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  ./src/EigenlayerQuery.sol:EigenlayerQuery \
  --constructor-args 0x8f86403A4DE0BB5791fa46B8e795C547942fE4Cf <Machine Hash>
```

Replace `<Machine Hash>` with machine hash from the previous step.

5. Call smart contract function to query operator balance

```bash
cast send \
  --rpc-url http://127.0.0.1:8545 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  0xA4899D35897033b927acFCf422bc745916139776 \
  "queryOperatorBalance(address,address)" 0x661344B4e1da1410F243335E4B249820070b8143 0x8198f5d8F8CfFE8f9C413d98a0A55aEB8ab9FbB7
```

6. Wait for transaction, sent on previous step, to successfully settle.
7. Call smart contract function to get operator balance

```bash
cast call \
  0xA4899D35897033b927acFCf422bc745916139776 "operatorBalance(address,address)" \
  0x661344B4e1da1410F243335E4B249820070b8143 0x8198f5d8F8CfFE8f9C413d98a0A55aEB8ab9FbB7
```

In the above bash commands:

- `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`: private key of anvil dev account
- `0x8f86403A4DE0BB5791fa46B8e795C547942fE4Cf`: address of coprocessor smart contract from devnet deployment
- `0x661344B4e1da1410F243335E4B249820070b8143`: address of test operator from devnet deployment
- `0x8198f5d8F8CfFE8f9C413d98a0A55aEB8ab9FbB7`: address of strategy ERC20 token from devnet deployment
