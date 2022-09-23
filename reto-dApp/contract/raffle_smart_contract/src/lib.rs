use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{near_bindgen};
use near_sdk::serde::{Deserialize,Serialize};
use near_sdk::{env, Promise, AccountId};


const NANOSECONS_IN_DAY: u64 = 86_400_000_000_000;

///Private raffle data, only accesible by the pub raffle and its limited operations
/// It contains the required information to work as a Raffle
/// Once configurated goes to true, in new method, the structure became unmutable at all
#[derive(Serialize, Deserialize,BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Raffle{
    pub id:u64,
    pub created_by: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub prize: u64,
    pub participants: Vec<AccountId>,
    pub creation_time_stamp: u64,
    pub open_days: u8,
    pub configurated: bool
}

impl Default for Raffle{
    fn default() -> Self{
        Raffle{
            id:0,
            created_by: String::new(),
            min_entry_price: 0,
            min_participants: 0,
            prize: 0,
            participants: Vec::<AccountId>::new(),
            creation_time_stamp: 0,
            open_days: 0,
            configurated: false
        }
    }
}


impl Raffle{
    pub fn new(min_entry_price: u64,min_participants: u64, prize: u64, open_days: u8) -> Self{
        Self {id: env::block_height(), created_by: env::signer_account_id().to_string(), min_entry_price, min_participants, prize, participants: Vec::<AccountId>::new(), 
            creation_time_stamp: env::block_timestamp(), open_days, configurated: true}
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PubRaffle{
    raffle: Raffle,
}

impl Default for PubRaffle{
    fn default() -> Self {
        Self { raffle: Raffle::new(0, 0, 0, 0) }
    }
}

#[near_bindgen]
impl PubRaffle {
    ///Creates a raffle if this contract doesn't have any raffle configured
    pub fn create_raffle(&mut self,min_entry_price: u64,min_participants: u64, prize: u64, open_days: u8){
        let raf = &self.raffle;
        if !raf.configurated{ //Never created, create a new one
            self.raffle = Raffle::new(min_entry_price, min_participants, prize, open_days);
            env::log_str("Raffle created successfully")
        }
        env::log_str("Raffle already created, is forbidden to create a new one")
    }

    ///Checks if the raffle is still open or not, if not, try to close the raffle if is
    /// not clossed already. Compare days between current day and creation day plus opened days 
    /// setted in the new raffle method
    pub fn check_status(&self){
        let current_time = env::block_timestamp();
        let open_days = &self.raffle.open_days;
        let days_in_nanosec = NANOSECONS_IN_DAY * (*open_days as u64);
        let expire_day = &self.raffle.creation_time_stamp + days_in_nanosec;
        if current_time > expire_day {
            env::log_str("Raffle closed")//To DO implement the close of the raffle
        }
        env::log_str("The Raffle is still open to participate :)")
    }
    #[payable]
    pub fn participate(&mut self) -> bool{
        assert!(
            env::attached_deposit() >= *&self.raffle.min_entry_price as u128,
            "The raffle minimum entry price is not reach"
        );

        //If the amount is reached, and the prize is different from 0, add the sender account to the ruffle and send the money
        if self.raffle.prize==0 {//Already closed
            env::log_str("Raffle closed");
            return false;
        }
        let new_participants = &mut self.raffle.participants;
        let mut signer_account = vec![env::signer_account_id()];
        new_participants.append(&mut signer_account);
        let acc : AccountId = self.raffle.created_by.parse().unwrap();
        Promise::new(acc).transfer(env::attached_deposit());

        true
    }
    
    //To Do: Close the contract and send the price to the winner or return everything back to the owners
}

