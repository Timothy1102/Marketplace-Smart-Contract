use crate::*;

pub trait NonFungibleTokenApprovalReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ListingArgs {
    pub sale_condition: SalePriceInYoctoNear,
    pub use_condition: UsePriceInYoctoNear,
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
    /**
    msg: {"sale_condition": "100000000000", "use_conditions:"1000000000"}
     */
    
}
