use crate::*;

const GAS_FOR_NFT_USES: Gas = 15_000_000_000_000;

#[ext_contract(nft_contract)]
pub trait NFTContract {
    fn nft_use_payout(
        &mut self,
        user_id: AccountId,
        token_id: TokenId,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[ext_contract(ext_self)]
pub trait MarketContract {
    fn resolve_use(&mut self, user_id: AccountId, price: U128) -> Promise;
}

#[near_bindgen]
impl Contract {
    
}
