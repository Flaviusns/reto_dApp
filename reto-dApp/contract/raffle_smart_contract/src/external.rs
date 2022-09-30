use near_sdk::{AccountId,ext_contract,};
use near_contract_standards::non_fungible_token::{TokenId,Token};

#[ext_contract(this_contract)]
trait CallBacks{
    fn query_get_token_from_nft_callback(&mut self) -> Option<Token>;
    fn query_nft_transfer(
        &self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

#[ext_contract(nft_in_near)]
trait NFTInNear {
    fn nft_transfer(
        &self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
    fn nft_metadata(&self);
    fn nft_token(&self,token_id: TokenId) -> Option<Token>;
}
