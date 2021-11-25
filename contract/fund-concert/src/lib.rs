use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, init, Timestamp};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ConcertCampaign {
    concerts: UnorderedMap<AccountId, Concert>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Concert {
    pub name: String,
    pub campaign_end: Timestamp,
    pub start_date: Timestamp,
    pub end_date: Timestamp,
    pub artist: AccountId,
    pub donors: UnorderedMap<AccountId, u128>,
    pub goal: u128,
}

impl Concert {
    #[init]
    pub fn new(name: String, campaign_end: Timestamp, start_date: Timestamp, end_date: Timestamp, artist: &AccountId, goal: u128) -> Self {
        Self {
            name,
            campaign_end,
            start_date,
            end_date,
            artist: artist.clone(),
            donors: UnorderedMap::new(0),
            goal,
        }
    }
    pub fn total_donations(&self) -> u128 {
        self.donors.values().sum()
    }

    pub fn is_funded(&self) -> bool {
        self.total_donations() >= self.goal
    }

    pub fn is_active(&self) -> bool {
        self.start_date <= env::block_timestamp() && env::block_timestamp() <= self.end_date
    }

    pub fn is_campaign_active(&self) -> bool {
        self.campaign_end >= env::block_timestamp()
    }

    pub fn add_donation(&mut self, donor: AccountId, amount: u128) {
        let current_donation = self.donors.get(&donor);
        match current_donation {
            Some(donation) => self.donors.insert(&donor, &(donation + &amount)),
            None => self.donors.insert(&donor, &amount),
        };
    }

    pub fn refund_donation(&mut self, donor: AccountId) -> Option<u128> {
        let current_donation = self.donors.get(&donor);
        match current_donation {
            Some(donation) => {
                self.donors.remove(&donor);
                Some(donation)
            }
            None => None
        }
    }
}

impl Default for ConcertCampaign {
    fn default() -> Self {
        Self {
            concerts: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl ConcertCampaign {
    #[payable]
    pub fn new_concert_campaign(
        &mut self,
        name: String,
        campaign_end: Timestamp,
        start_date: Timestamp,
        end_date: Timestamp,
        artist: AccountId,
        goal: u128
    ) {
        let concert = Concert::new(name, campaign_end, start_date, end_date, &artist, goal);
        self.concerts.insert(&artist, &concert);
    }

    #[payable]
    pub fn concert_add_donation(&mut self, concert_id: AccountId, donor: AccountId, donation: u128) -> bool {
        let concert = self.concerts.get(&concert_id);
        match concert {
            Some(mut concert) => {
                if concert.is_active() {
                    concert.add_donation(donor, donation);
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    #[payable]
    pub fn concert_refund_donation(&mut self, concert_id: AccountId, donor: AccountId) -> Option<u128> {
        let concert = self.concerts.get(&concert_id);
        match concert {
            Some(mut concert) => {
                if concert.is_active() {
                    let amount = concert.refund_donation(donor);
                    match amount {
                        Some(amount) => {
                            // TODO: refund the donation
                            Some(amount)
                        },
                        None => None
                    }
                } else {
                    None
                }
            }
            None => None
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
