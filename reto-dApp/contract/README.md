# Raffle Manager  

Here we can find the raffle manager, which can create raffles as lock contracts.

First to build the contract:

`cargo build --target wasm32-unknown-unknown --release`

Then publish it and interact with it:

```
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/raffle_manager.wasm

near call subcuenta.flaviusstan.testnet create_raffle '{"description": "Concurso demo", "min_entry_price":1, "min_participants": 1, "nft_account":"hash_3", "prize": "hash_3", "open_days":10}' --accountId flaviusstan.testnet --amount 10
```

The amount must be higher enough to handle the lock contract, so at least 10 nears are required

The with the generated lock contract, you can interact with the subaccount created, for example: `hash_1.devaccount`

For more information on how to handle the sub account, go to the readme in the raffle smart contract directory

