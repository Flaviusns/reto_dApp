# Raffle Smart Contract

This is the directory of the raffle smart contract, first build it:

`cargo build --target wasm32-unknown-unknown --release`

The publish it:
`near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/raffle_smart_contract.wasm`

First of all you need to create your raffle, to do that use the create raffle function like this:

```
near call hash_3.subcuenta.flaviusstan.testnet create_raffle '{"description": "Prueba de concurso", "min_entry_price": 1, "min_participants": 1, "prize": "hash_1","nft_account":"dev-1665422283302-19334337854552", "open_days":10}' --accountId flaviusstan.testnet

near call dev-1665423540034-14670949817966 create_raffle '{"description": "Prueba de concurso", "min_entry_price": 1, "min_participants": 1, "prize": "hash_4","nft_account":"dev-1665423488308-17865810409193", "open_days":10}' --accountId flaviusstan.testnet
```

After that, you can participate, checks the participant list and also raffle detais:
```
near call your_raffle_account participate '{}' --accountId your_account --amount 1

near view your_raffle_account get_raffle_data '{}'
near view your_raffle_account check_if_raffle_is_open '{}'
```