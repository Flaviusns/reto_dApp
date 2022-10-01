cargo build --target wasm32-unknown-unknown --release
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/raffle_smart_contract.wasm
near call dev-1664546028201-88028282449571 create_raffle '{"description": "Prueba de concurso", "min_entry_price": 1, "min_participants": 1, "prize": "hash_1","nft_account":"dev-1664546003165-18085703894003", "open_days":10}' --accountId flaviusstan.testnet
near view dev-1664546028201-88028282449571 get_expire_app '{}' 
near view dev-1664546028201-88028282449571 get_raffle_data '{}'
near view dev-1664546028201-88028282449571 check_if_raffle_is_open '{}'
near call dev-1664546028201-88028282449571 participate '{}' --accountId flaviusstan.testnet --amount 1
near view dev-1664546028201-88028282449571 get_participants '{}'

##Comandos en el manager
near call dev-1664622659190-79306488887267 create_raffle '{"description": "Concurso de prueba", "min_entry_price":1, "min_participants": 1, "prize": "hash_1"}' --accountId flaviusstan.testnet --amount 1

near call hash_1.dev-1664622659190-79306488887267 create_raffle '{"description": "Prueba de concurso", "min_entry_price": 1, "min_participants": 1, "prize": "hash_1","nft_account":"dev-1664546003165-18085703894003", "open_days":10}' --accountId flaviusstan.testnet 

1_000_000_000_000_000_000_000_000