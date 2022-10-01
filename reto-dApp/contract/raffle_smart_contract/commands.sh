cargo build --target wasm32-unknown-unknown --release
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/raffle_smart_contract.wasm
near call dev-1664298233925-98009146220607 create_raffle '{"min_entry_price": 1, "min_participants": 1, "prize": 1, "open_days":2}' --accountId flaviusstan.testnet
near view dev-1664298233925-98009146220607 get_expire_app '{}' 