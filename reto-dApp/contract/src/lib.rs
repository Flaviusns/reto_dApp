use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen};
use near_sdk::serde::{Deserialize,Serialize};
use near_sdk::{env};

//const MIN_STORAGE: Balance = 1_000_000_000_000_000_000_000; //0.001Ⓝ

#[derive(Serialize, Deserialize,BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Raffle{
    pub created_by: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub prize: String,
}

impl Default for Raffle{
    fn default() -> Self{
        Raffle{
            created_by: String::new(),
            min_entry_price: 0,
            min_participants: 0,
            prize: String::new(),
        }
    }
}

impl Raffle{
    pub fn new(min_entry_price: u64,min_participants: u64, prize: String) -> Self{
        Self {created_by: env::signer_account_id().to_string(), min_entry_price, min_participants, prize}
    }
}

///Manager that handles all the raffles and also give its address to interact with
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RiffleManager{
    raffles: UnorderedMap<String, Raffle>
}

impl Default for RiffleManager{
    fn default() -> Self{
        Self { raffles: UnorderedMap::new(b"e".to_vec()) }
    }
}

#[near_bindgen]
impl RiffleManager{
    ///Get the entire list of raffles
    pub fn get_list_raffle(&self) -> Vec<(String, Raffle)>{
        self.raffles.to_vec()
    }
    ///Create a template raffle just for testing purposes
    pub fn create_raffle(&mut self, min_entry_price: u64,min_participants: u64, prize: String){
        let raffle = Raffle::new(min_entry_price, min_participants, prize);

        self.raffles.insert(&raffle.prize, &raffle);
        env::log_str("Raffle created successfully");
    }
}