#!/bin/sh

#OLD_RAFFLE_CONTRACT_ID=dev-1664622659190-79306488887267
RAFFLE_MANAGER_CONTRACT_ID=subcuenta.flaviusstan.testnet
ACCOUNT_SIGN=joardev.testnet
DEPOSIT_AMOUNT=10

# Raffle Manager
#near call dev-1664622659190-79306488887267 create_raffle '{"description": "test from near cli", "min_entry_price": 1, "min_participants": 1, "prize": "testRaffleCLI","nft_account":"testRaffleCLI", "open_days":1}' --accountId joardev.testnet --deposit 10

#1st try 
#near call $RAFFLE_MANAGER_CONTRACT_ID create_raffle '{"description": "test from near cli", "min_entry_price": 1, "min_participants": 1, "prize": "testRaffleCLI","nft_account":"testRaffleCLI", "open_days":1}' --accountId $ACCOUNT_SIGN --deposit $DEPOSIT_AMOUNT
#2nd try
#near call $RAFFLE_MANAGER_CONTRACT_ID create_raffle '{"description": "raffle test", "min_entry_price": 1, "min_participants": 2, "prize": "nftHash","nft_account":"nftHash", "open_days":2}' --accountId $ACCOUNT_SIGN --deposit $DEPOSIT_AMOUNT
#near view $RAFFLE_MANAGER_CONTRACT_ID get_list_raffle '{}'

RAFFLE_CONTRACT_ID=hash_2.subcuenta.flaviusstan.testnet
PARTICIPATE_DEPOSIT_AMOUNT=1

# Raffle
#near call $RAFFLE_CONTRACT_ID create_raffle '{"description": "test raffle from frontend","min_entry_price": 1,"min_participants": 1,"prize": "from_frontend","nft_account": "from_frontend","open_days": 1}' --accountId joardev.testnet
# return true
near call $RAFFLE_CONTRACT_ID check_if_raffle_is_open '{}' --accountId $ACCOUNT_SIGN
#near call $RAFFLE_CONTRACT_ID participate '{}' --accountId $ACCOUNT_SIGN --deposit $PARTICIPATE_DEPOSIT_AMOUNT

#near view $RAFFLE_CONTRACT_ID get_raffle '{}' --accountId $ACCOUNT_SIGN

# get array of participants
#near view $RAFFLE_CONTRACT_ID get_participants '{}' --accountId $ACCOUNT_SIGN
# get raffle info (properties)
#near view $RAFFLE_CONTRACT_ID get_raffle '{}' --accountId $ACCOUNT_SIGN