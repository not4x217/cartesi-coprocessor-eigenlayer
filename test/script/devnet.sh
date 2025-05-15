#!/usr/bin/env bash

cartesi-coprocessor publish --network devnet
cartesi-coprocessor address-book

cd contracts/

forge create --broadcast \
  --rpc-url http://127.0.0.1:8545 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  ./src/EigenlayerQuery.sol:EigenlayerQuery \
  --constructor-args 0x8f86403A4DE0BB5791fa46B8e795C547942fE4Cf 0xcb74f1bb33857d6c644668129b8bb1313d52ba44e88441e631f8c0b30fb9f515

#cast call 0x8198f5d8F8CfFE8f9C413d98a0A55aEB8ab9FbB7 "balanceOf(address)" 0x661344B4e1da1410F243335E4B249820070b8143

cast send \
  --rpc-url http://127.0.0.1:8545 \
  --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
  0xA4899D35897033b927acFCf422bc745916139776 \
  "queryOperatorBalance(address,address)" 0x661344B4e1da1410F243335E4B249820070b8143 0x8198f5d8F8CfFE8f9C413d98a0A55aEB8ab9FbB7