use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen};
use near_sdk::serde::{Deserialize,Serialize};
use near_sdk::{env, Promise, AccountId};

//const MIN_STORAGE: Balance = 1_000_000_000_000_000_000_000; //0.001Ⓝ

#[derive(Serialize, Deserialize,BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Raffle{
    pub id:u64,
    pub created_by: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub prize: String,
    pub participants: u128,
}

impl Default for Raffle{
    fn default() -> Self{
        Raffle{
            id:0,
            created_by: String::new(),
            min_entry_price: 0,
            min_participants: 0,
            prize: String::new(),
            participants: 0,
        }
    }
}

impl Raffle{
    pub fn new(min_entry_price: u64,min_participants: u64, prize: String) -> Self{
        Self {id: env::block_height(), created_by: env::signer_account_id().to_string(), min_entry_price, min_participants, prize, participants: 0}
    }
}

///Manager that handles all the raffles and also give its address to interact with
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RiffleManager{
    raffles: UnorderedMap<u64, Raffle>
}

impl Default for RiffleManager{
    fn default() -> Self{
        Self { raffles: UnorderedMap::new(b"e".to_vec()) }
    }
}

//Implements all the related functionalities with the manager (get, getall)