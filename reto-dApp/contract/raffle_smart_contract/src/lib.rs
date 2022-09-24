use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{near_bindgen, BlockHeight};
use near_sdk::{env, Promise, AccountId};
use chrono::prelude::*;

const NANOSECONS_IN_DAY: u64 = 86_400_000_000_000;

///Private raffle data, only accesible by the pub raffle and its limited operations
/// It contains the required information to work as a Raffle
/// Once configurated goes to true, in new method, the structure became unmutable at all
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle{
    pub id:u64,
    pub created_by: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub prize: u64,
    pub participants: UnorderedMap<AccountId,u128>,
    pub participants_order: Vec<AccountId>,
    pub creation_time_stamp: u64,
    pub open_days: u8,
    pub configurated: bool
}

impl Default for Raffle{
    fn default() -> Self {
        Raffle {
            id: 0,
            created_by: String::from(""),
            min_entry_price: 0,
            min_participants: 0,
            prize: 0,
            participants: UnorderedMap::new(b"e".to_vec()),
            participants_order: vec![],
            creation_time_stamp: 0,
            open_days: 0,
            configurated: false
        }
    }
}

impl Raffle{
    pub fn new(id: BlockHeight, min_entry_price: u64,min_participants: u64, prize: u64, open_days: u8, time_stamp: u64) -> Self{
        Self {id, created_by: env::signer_account_id().to_string(), min_entry_price, min_participants, prize, 
            participants: UnorderedMap::new(b"e".to_vec()), participants_order: vec![],
            creation_time_stamp: time_stamp, open_days, configurated: true}
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PubRaffle{
    raffle: Raffle,
}

impl Default for PubRaffle{ //I saw a document indicatin defualt is not recommended but it don't compile if not
     fn default() -> Self {
         Self { raffle: Raffle::new(0, 0, 0, 0,0,0) }
     }
 }

#[near_bindgen]
impl PubRaffle {
    ///This create the raffle, this method only can be called once
    //#[init]
    // pub fn create_raffle(min_entry_price: u64,min_participants: u64, prize: u64, open_days: u8) -> Self{
    //     env::log_str("Preparing to start up the raffle");
    //     Self {
    //         raffle: Raffle::new(min_entry_price, min_participants, prize, open_days)
    //     }
    // }

    pub fn create_raffle(&mut self, min_entry_price: u64,min_participants: u64, prize: u64, open_days: u8){
        self.raffle = Raffle::new(env::block_height(), min_entry_price, min_participants, prize, open_days, 0)
    }

    ///Checks if the raffle is still open or not, if not, try to close the raffle if is
    /// not clossed already. Compare days between current day and creation day plus opened days 
    /// setted in the new raffle method
    pub fn check_status(&mut self){
        let current_time = env::block_timestamp();
        let open_days = &self.raffle.open_days;
        let days_in_nanosec = NANOSECONS_IN_DAY * (*open_days as u64);
        let expire_day = &self.raffle.creation_time_stamp + days_in_nanosec;
        if current_time > expire_day {
            self.close_raffle();
            return env::log_str("Raffle closed")
        }
        let naive = NaiveDateTime::from_timestamp(0, expire_day.try_into().unwrap());
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        env::log_str(format!("The Raffle is still open to participate, expire day: {}", newdate).as_str());
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
        self.raffle.participants.insert(&env::signer_account_id(), &env::attached_deposit()); //To know how much give to the price
        //Add also the order to being able to iterate
        //self.raffle.participants_order.push(&env::signer_account_id());
        self.raffle.participants_order.push(env::signer_account_id());
        Promise::new(env::current_account_id()).transfer(env::attached_deposit());
        true
    }
    
    fn close_raffle(&mut self){
        let raf = &mut self.raffle;
        let len = raf.participants.len();
        if len >= raf.min_participants.try_into().unwrap(){//the raffle can give a price
            let current_day = env::block_timestamp();
            let winner_index = current_day % len; //Get the winner as mod, it will be better with a random function but...
            let winner = &self.raffle.participants_order[winner_index as usize];
            let winner_acc: AccountId = String::from(winner.to_string()).parse().unwrap();
            Promise::new(winner_acc).transfer(env::account_balance());
        }else{
            //Return the money to everyone and the rest to the raffle's creator
            for part in self.raffle.participants_order.iter(){
                let donated = &self.raffle.participants.get(&part).unwrap();
                let parti_acc: AccountId = String::from(part.to_string()).parse().unwrap(); 
                Promise::new(parti_acc).transfer(*donated);
            }
            //Then return the rest to the create by
            let acc : AccountId = String::from(&self.raffle.created_by).parse().unwrap();
            Promise::new(acc).transfer(env::account_balance());
        }

    }
}

