use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, Balance, Promise};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

const YOCTO_MIN: Balance = 3_100_00_000_000_000_000_000_000;
const RAFFLE_CODE: &[u8] = include_bytes!("../raffle_smart_contract.wasm");

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Raffle {
    pub created_by: String,
    pub description: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub nft_account: String,
    pub prize: String,
    pub account: String,
    pub open_days: u8
}

impl Default for Raffle {
    fn default() -> Self {
        Raffle {
            created_by: String::new(),
            description: String::new(),
            min_entry_price: 0,
            min_participants: 0,
            nft_account: String::new(),
            prize: String::new(),
            account: String::new(),
            open_days: 0
        }
    }
}

impl Raffle {
    pub fn new(
        description: String,
        min_entry_price: u64,
        min_participants: u64,
        prize: String,
        nft_account: String,
        account: String,
        open_days: u8
    ) -> Self {
        Self {
            description,
            created_by: env::signer_account_id().to_string(),
            min_entry_price,
            min_participants,
            nft_account,
            prize,
            account,
            open_days
        }
    }
}

///Manager that handles all the raffles and also give its address to interact with
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RiffleManager {
    raffles: UnorderedMap<String, Raffle>,
}

impl Default for RiffleManager {
    fn default() -> Self {
        Self {
            raffles: UnorderedMap::new(b"e".to_vec()),
        }
    }
}

#[near_bindgen]
impl RiffleManager {
    ///Get the entire list of raffles
    pub fn get_list_raffle(&self) -> Vec<(String, Raffle)> {
        self.raffles.to_vec()
    }
    ///Create a raffle as lock contract and returns the contract address to interact with
    #[payable]
    pub fn create_raffle(//10 Near mínimo
        &mut self,
        description: String,
        min_entry_price: u64,
        min_participants: u64,
        nft_account: String,
        prize: String,
        open_days: u8,
    ) -> String {
        assert!(
            env::attached_deposit() >= (10*YOCTO_MIN),
            "The raffle minimum entry price is not reach or the raffle is not open to participate in it"
        );
        let account_id = prize.clone() + "." + &env::current_account_id().to_string();
        let raffle = Raffle::new(description, min_entry_price, min_participants, nft_account,prize,account_id.clone(),open_days);
        Promise::new(account_id.parse().unwrap())
            .create_account()
            .transfer(env::attached_deposit())
            .deploy_contract(RAFFLE_CODE.to_vec());
        self.raffles.insert(&raffle.prize, &raffle);
        env::log_str("Raffle created successfully");
        account_id //hash.cuentamanager.testnet
    }
}
