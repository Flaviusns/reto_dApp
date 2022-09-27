use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;
use near_sdk::{env, AccountId, Promise};

const NANOSECONS_IN_DAY: u64 = 86_400_000_000_000;

///Private raffle data, only accesible by the pub raffle and its limited operations
/// It contains the required information to work as a Raffle
/// Once configurated goes to true, in new method, the structure became unmutable at all
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle {
    pub id: u64,
    pub created_by: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub prize: u64,
    pub participants: UnorderedMap<AccountId, u128>,
    pub participants_order: Vec<AccountId>,
    pub creation_time_stamp: u64,
    pub open_days: u8,
    pub configurated: bool,
    pub closed: bool
}

impl Default for Raffle {
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
            configurated: false,
            closed: false
        }
    }
}

impl Raffle {
    pub fn new(min_entry_price: u64, min_participants: u64, prize: u64, open_days: u8) -> Self {
        Self {
            id: env::block_height(),
            created_by: env::signer_account_id().to_string(),
            min_entry_price,
            min_participants,
            prize,
            participants: UnorderedMap::new(b"e".to_vec()),
            participants_order: vec![],
            creation_time_stamp: 0,
            open_days,
            configurated: true,
            closed: false
        }
    }

    pub fn new_default(min_entry_price: u64, min_participants: u64, prize: u64, open_days: u8) -> Self {
        Self {
            id: env::block_height(),
            created_by: env::signer_account_id().to_string(),
            min_entry_price,
            min_participants,
            prize,
            participants: UnorderedMap::new(b"e".to_vec()),
            participants_order: vec![],
            creation_time_stamp: 0,
            open_days,
            configurated: false,
            closed: false
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PubRaffle {
    raffle: Raffle,
}

impl Default for PubRaffle {
    fn default() -> Self {
        Self {
            raffle: Raffle::new_default(0, 0, 0, 0),
        }
    }
}

#[near_bindgen]
impl PubRaffle {
    ///This create the raffle, this method only can be called once
    //#[init]
    pub fn create_raffle(
        &mut self,
        min_entry_price: u64,
        min_participants: u64,
        prize: u64,
        open_days: u8,
    ) -> bool {
        if self.raffle.configurated {
            env::log_str(
                format!("Raffle already configurated: {}", &self.raffle.configurated).as_str(),
            );
            false
        } else {
            self.raffle = Raffle::new(min_entry_price, min_participants, prize, open_days);
            env::log_str("Raffle configurated successfully");
            true
        }
    }


    pub fn get_raffle_data(&self) {
        let current_data = format!("The raffle data is the following: created by: {}, min_entry_price:{}, min_participants: {}, price: {},configured:{}",
    self.raffle.created_by, self.raffle.min_entry_price, self.raffle.min_participants, self.raffle.prize, self.raffle.configurated);
        env::log_str(&current_data.as_str())
    }

    pub fn get_expire_app(&self) -> u64 {
        let open_days = &self.raffle.open_days;
        let days_in_nanosec = NANOSECONS_IN_DAY * (*open_days as u64);
        let expire_day = &self.raffle.creation_time_stamp + days_in_nanosec;
        expire_day
    }
    ///Checks if the raffle is still open or not, if not, try to close the raffle if is
    /// not clossed already. Compare days between current day and creation day plus opened days
    /// setted in the new raffle method
    pub fn check_status(&mut self) {
        let current_time = env::block_timestamp();
        let open_days = &self.raffle.open_days;
        let days_in_nanosec = NANOSECONS_IN_DAY * (*open_days as u64);
        let expire_day = &self.raffle.creation_time_stamp + days_in_nanosec;
        if current_time > expire_day {
            if !self.raffle.closed{ //Is still open, we need to close it
                self.close_raffle();
            }
            return env::log_str("Raffle closed");
        }
        env::log_str(
            format!(
                "The Raffle is still open to participate, expire day: {}",
                expire_day
            )
            .as_str(),
        );
    }

    #[payable]
    pub fn participate(&mut self) -> bool {
        assert!(
            env::attached_deposit() >= *&self.raffle.min_entry_price as u128,
            "The raffle minimum entry price is not reach"
        );

        //If the amount is reached, and the prize is different from 0, add the sender account to the ruffle and send the money
        if self.raffle.prize == 0 {
            //Already closed
            env::log_str("Raffle closed");
            return false;
        }
        self.raffle
            .participants
            .insert(&env::signer_account_id(), &env::attached_deposit()); //To know how much give to the price
                                                                          //Add also the order to being able to iterate
                                                                          //self.raffle.participants_order.push(&env::signer_account_id());
        self.raffle
            .participants_order
            .push(env::signer_account_id());
        Promise::new(env::current_account_id()).transfer(env::attached_deposit());
        true
    }
    ///Close the raffle, first checks if the min conditions was reach, if not, send back the money, else give all the money to the winner
    fn close_raffle(&mut self) {
        let raf = &mut self.raffle;
        let len = raf.participants.len();
        if len >= raf.min_participants.try_into().unwrap() {
            //the raffle can give a price
            env::log_str("Raffle close as selecting winner");
            let current_day = env::block_timestamp();
            let winner_index = current_day % len; //Get the winner as mod, it will be better with a random function but...
            let winner = &self.raffle.participants_order[winner_index as usize];
            let winner_acc: AccountId = String::from(winner.to_string()).parse().unwrap();
            env::log_str("He obtenido ganar");
            let win_format = format!("Ganador: {}", &winner_acc.as_str());
            env::log_str(&win_format);
            Promise::new(winner_acc).transfer(env::account_balance());
        } else {
            //Return the money to everyone and the rest to the raffle's creator
            env::log_str("Raffle close as not selecting winner");
            for part in self.raffle.participants_order.iter() {
                let donated = &self.raffle.participants.get(&part).unwrap();
                let parti_acc: AccountId = String::from(part.to_string()).parse().unwrap();
                Promise::new(parti_acc).transfer(*donated);
            }
            env::log_str("For ended");
            //Then return the rest to the create by
            let acc: AccountId = String::from(&self.raffle.created_by).parse().unwrap();
            Promise::new(acc).transfer(env::account_balance());
        }

        self.raffle.closed = true;
    }
}

#[cfg(not(target_arch = "wasm32"))]
 #[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env};
    fn get_context() {
        let mut builder = VMContextBuilder::new();
        builder.block_timestamp(1);
        builder.block_index(1);
        builder.attached_deposit(1);
        testing_env!(builder.build());
    }
    // Test 1
    #[test]
    fn get_expire_day() {
        get_context();
        let mut contract: PubRaffle = PubRaffle {
            raffle: Raffle::new(0, 0, 0, 0)
        };
        contract.create_raffle(1, 1, 1, 1);
        assert_eq!(86400000000000, contract.get_expire_app());
    }
    // Test 2
    #[test]
    fn get_winner() {
        get_context();
        let mut contract: PubRaffle = PubRaffle {
            raffle: Raffle::new(0, 0, 0, 0)
        };
        contract.create_raffle(1, 1, 1, 0);
        contract.participate();
        contract.check_status();
        assert_eq!(0, contract.get_expire_app());
    }
}
