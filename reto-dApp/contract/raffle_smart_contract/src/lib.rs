use near_contract_standards::non_fungible_token::Token;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;
use near_sdk::{env, AccountId, Gas, Promise, PromiseError};
pub mod external;
pub use crate::external::*;

const NANOSECONS_IN_DAY: u64 = 86_400_000_000_000;
pub const TGAS: u64 = 1_000_000_000_000;

///Private raffle data, only accesible by the pub raffle and its limited operations
/// It contains the required information to work as a Raffle
/// Once configurated goes to true, in new method, the structure became unmutable at all
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle {
    pub id: u64,
    pub created_by: String,
    pub min_entry_price: u64,
    pub min_participants: u64,
    pub prize: String, //TokenID
    pub nft_account: String,
    pub participants: UnorderedMap<AccountId, u128>,
    pub participants_order: Vec<AccountId>,
    pub creation_time_stamp: u64,
    pub open_days: u8,
    pub configurated: bool,
    pub closed: bool,
}

impl Default for Raffle {
    fn default() -> Self {
        Raffle {
            id: 0,
            created_by: String::from(""),
            min_entry_price: 0,
            min_participants: 0,
            prize: String::from(""), //Token_Id
            nft_account: String::from(""),
            participants: UnorderedMap::new(b"e".to_vec()),
            participants_order: vec![],
            creation_time_stamp: 0,
            open_days: 0,
            configurated: false,
            closed: true,
        }
    }
}

impl Raffle {
    pub fn new(
        min_entry_price: u64,
        min_participants: u64,
        prize: String,
        nft_account: String,
        open_days: u8,
    ) -> Self {
        Self {
            id: env::block_height(),
            created_by: env::signer_account_id().to_string(),
            min_entry_price,
            min_participants,
            prize,
            nft_account,
            participants: UnorderedMap::new(b"e".to_vec()),
            participants_order: vec![],
            creation_time_stamp: 0,
            open_days,
            configurated: true,
            closed: true,
        }
    }

    pub fn new_default(
        min_entry_price: u64,
        min_participants: u64,
        prize: String,
        nft_account: String,
        open_days: u8,
    ) -> Self {
        Self {
            id: env::block_height(),
            created_by: env::signer_account_id().to_string(),
            min_entry_price,
            min_participants,
            prize,
            nft_account,
            participants: UnorderedMap::new(b"e".to_vec()),
            participants_order: vec![],
            creation_time_stamp: 0,
            open_days,
            configurated: false,
            closed: true,
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
            raffle: Raffle::new_default(0, 0, String::from("_"), String::from("_"), 0),
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
        prize: String,
        nft_account: String,
        open_days: u8,
    ) -> bool {
        if self.raffle.configurated {
            env::log_str(
                format!("Raffle already configurated: {}", &self.raffle.configurated).as_str(),
            );
            false
        } else {
            self.raffle = Raffle::new(
                min_entry_price,
                min_participants,
                prize,
                nft_account,
                open_days,
            );
            env::log_str("Raffle configurated, now its waiting for being the owner of the NFT, transfer it to start the raffle");
            true
        }
    }

    pub fn get_raffle_data(&self) {
        let current_data = format!("The raffle data is the following: created by: {}, min_entry_price:{}, min_participants: {}, prize: {}, NFT_owner: {}, configured:{}, closed:{}",
    self.raffle.created_by, self.raffle.min_entry_price, self.raffle.min_participants, self.raffle.prize, self.raffle.nft_account, self.raffle.configurated, self.raffle.closed);
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
            if !self.raffle.closed {
                //Is still open, we need to close it
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
    
    ///This function checks if the owner of the NFT is the current account and if it is,
    /// enable the raffle to participate
    #[private] // Public - but only callable by env::current_account_id()
    pub fn query_get_token_from_nft_callback(
        &mut self,
        #[callback_result] call_result: Result<Token, PromiseError>,
    ) -> Option<Token> {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            return None;
        } else {
            let token: Token = call_result.unwrap();
            if token.owner_id == env::current_account_id() {
                self.raffle.closed = false;
            }
            return Some(token);
        }
    }

    pub fn check_if_raffle_is_open(&self) -> Promise {
        env::log_str(&self.raffle.nft_account);
        let nft_account: AccountId = String::from(&self.raffle.nft_account).parse().unwrap();
        let token_data = nft_in_near::ext(nft_account)
            .with_static_gas(Gas(5 * TGAS))
            .nft_token(self.raffle.prize.clone());

        return token_data.then({
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .query_get_token_from_nft_callback()
        });
    }

    #[payable]
    pub fn participate(&mut self) {
        assert!(
            env::attached_deposit() >= *&self.raffle.min_entry_price as u128 && !*&self.raffle.closed,
            "The raffle minimum entry price is not reach or the raffle is not open to participate in it"
        );
        if !&self.raffle.closed {
            self.raffle
                .participants
                .insert(&env::signer_account_id(), &env::attached_deposit()); //To know how much give to the price
                                                                              //Add also the order to being able to iterate
                                                                              //self.raffle.participants_order.push(&env::signer_account_id());
            self.raffle
                .participants_order
                .push(env::signer_account_id());
            env::log_str("You has been added as a participant into this raffle");
            Promise::new(env::current_account_id()).transfer(env::attached_deposit());
        }
    }

    pub fn get_participants(&self) -> Vec<AccountId> {
        self.raffle.participants_order.to_vec()
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
            let winner = &raf.participants_order[winner_index as usize];
            let winner_acc: AccountId = String::from(winner.to_string()).parse().unwrap();
            env::log_str("He obtenido ganar");
            let win_format = format!("Winner: {}", &winner_acc.as_str());
            env::log_str(&win_format);
            //Transfering the NFT to the winner
            let nft_account: AccountId = String::from(&raf.nft_account.to_string()).parse().unwrap();
            nft_in_near::ext(nft_account)
                .with_static_gas(Gas(1))
                .with_attached_deposit(1)
                .nft_transfer(winner_acc, self.raffle.prize.clone(), None, None);
            //Give the nears to the creator of this raffle
            let acc: AccountId = String::from(&self.raffle.created_by).parse().unwrap();
            Promise::new(acc).transfer(env::account_balance());
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

    pub fn close_raffle_with_winner(&mut self, winner: String) {
        let winner_acc: AccountId = String::from(winner.to_string()).parse().unwrap();
        env::log_str("He obtenido ganar");
        let win_format = format!("Winner: {}", &winner_acc.as_str());
        env::log_str(&win_format);
        //Transfering the NFT
        let nft_account: AccountId = String::from(&self.raffle.nft_account).parse().unwrap();
        nft_in_near::ext(nft_account)
            .with_static_gas(Gas(1))
            .with_attached_deposit(1)
            .nft_transfer(winner_acc, self.raffle.prize.clone(), None, None);

        self.raffle.closed = true;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
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
            raffle: Raffle::new(0, 0, String::from(""), String::from(""), 0),
        };
        contract.create_raffle(1, 1, String::from(""), String::from(""), 1);
        assert_eq!(86400000000000, contract.get_expire_app());
    }
    // Test 2
    #[test]
    fn get_winner() {
        get_context();
        let mut contract: PubRaffle = PubRaffle {
            raffle: Raffle::new(0, 0, String::from(""), String::from(""), 0),
        };
        contract.create_raffle(1, 1, String::from(""), String::from(""), 0);
        contract.participate();
        contract.check_status();
        assert_eq!(0, contract.get_expire_app());
    }
}
