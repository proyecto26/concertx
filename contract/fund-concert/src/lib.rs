use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, init, Timestamp, log, Promise};

near_sdk::setup_alloc!();

const STORAGE_COST: u128 = 5870000000000000000000;

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
        goal: u128
    ) -> Concert{
        log!("Creating new concert campaign");
        let artist = env::signer_account_id();
        let concert = Concert::new(name, campaign_end, start_date, end_date, &artist, goal);
        self.concerts.insert(&artist, &concert);
        log!("New concert campaign created!");
        concert
    }

    #[payable]
    pub fn concert_add_donation(&mut self, concert_id: AccountId) -> bool {
        let donor = env::signer_account_id();
        let donation = env::attached_deposit();
        let concert = self.concerts.get(&concert_id);
        match concert {
            Some(mut concert) => {
                if concert.is_campaign_active() {
                    let account_id = concert.artist.to_string();
                    // TODO: Check if the account is the same as the artist
                    // TODO: Substract the storage cost
                    // Promise::new(account_id).transfer(donation);
                    log!("Donation added to concert {}.", donation);
                    concert.add_donation(donor, donation);
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    const DONATION_VALUE: u128 = 5870000000000000000000000000000000000;

    fn get_context(input: Vec<u8>, is_view: bool, is_donor: bool) -> VMContext {
        let signer_account_id = if is_donor {
            "alice.near".to_string()
        } else {
            "bob.near".to_string()
        };
        VMContext {
            current_account_id: signer_account_id.to_string(),
            signer_account_id,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: DONATION_VALUE,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn new_concert_campaign() {
        let context = get_context(vec![], false, false);
        testing_env!(context);
        let mut contract = ConcertCampaign::default();
        let campaign_created = contract.new_concert_campaign("Concert 1".to_string(), 100, 0, 100, 100);
        assert_eq!(campaign_created.name, "Concert 1");
        assert_eq!(campaign_created.artist, "bob.near");
        assert_eq!(contract.concerts.len(), 1);
    }

    #[test]
    fn concert_add_donation() {
        let context = get_context(vec![], false, false);
        testing_env!(context);
        let mut contract = ConcertCampaign::default();
        let _campaign_created = contract.new_concert_campaign("Concert 1".to_string(), 100, 0, 100, 100);

        let context = get_context(vec![], false, true);
        testing_env!(context);

        let artist = "bob.near".to_string();
        let donation_complete = contract.concert_add_donation(artist);
        let artist = "bob.near".to_string();

        assert_eq!(donation_complete, true);
        assert_eq!(contract.concerts.len(), 1);

        // TODO: Check if the donation was added to the concert
        assert_eq!(contract.concerts.get(&artist).unwrap().donors.len(), 1);
        assert_eq!(contract.concerts.get(&artist).unwrap().total_donations(), DONATION_VALUE);
        
    }
}
