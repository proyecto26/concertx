use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, BorshStorageKey};

setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    FollowersIds,
    FollowingIds,
    Records,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Relationships {
    followers: UnorderedSet<AccountId>,
    following: UnorderedSet<AccountId>,
}

impl Default for Relationships {
    fn default() -> Self {
        Self {
            followers: UnorderedSet::new(StorageKey::FollowersIds),
            following: UnorderedSet::new(StorageKey::FollowingIds),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Followers {
    records: LookupMap<AccountId, Relationships>,
}

impl Default for Followers {
    fn default() -> Self {
        Self {
            records: LookupMap::new(StorageKey::Records),
        }
    }
}

#[near_bindgen]
impl Followers {
    pub fn follow(&mut self, follow_account_id: AccountId) {
        let account_id = env::signer_account_id();
        let mut signer_relationships = self.get_relationships(&account_id);
        let mut follow_account_relationships = self.get_relationships(&follow_account_id);
        signer_relationships.following.insert(&follow_account_id);
        follow_account_relationships.followers.insert(&account_id);
        self.records.insert(&account_id, &signer_relationships);
        self.records
            .insert(&follow_account_id, &follow_account_relationships);
    }

    pub fn get_followers(&mut self, account_id: AccountId) -> UnorderedSet<AccountId> {
        self.get_relationships(&account_id).followers
    }

    pub fn get_following(&mut self, account_id: AccountId) -> UnorderedSet<AccountId> {
        self.get_relationships(&account_id).following
    }

    fn get_relationships(&mut self, account_id: &AccountId) -> Relationships {
        match self.records.get(&account_id) {
            Some(relations) => relations,
            None => Relationships::default(),
        }
    }
}
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
            epoch_height: 19,
        }
    }

    #[test]
    fn follow_then_check_followers() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Followers::default();
        let test_account_id = "testing_near".to_string();
        let mut test_account_followers = contract.get_followers(test_account_id.clone());

        assert_eq!(
            test_account_followers.len(),
            0,
            "Test account followers should be empty at the beginning"
        );
        contract.follow(test_account_id.clone());
        test_account_followers = contract.get_followers(test_account_id.clone());
        assert_eq!(
            test_account_followers.len(),
            1,
            "Test account followers should have one element after execution of follow function"
        );
    }

    #[test]
    fn follow_then_check_following() {
        let context = get_context(vec![], false);
        let signer_account_id = context.signer_account_id.clone();
        testing_env!(context);
        let mut contract = Followers::default();
        let test_account_id = "testing_near".to_string();
        let mut signer_following = contract.get_following(signer_account_id.clone());

        assert_eq!(
            signer_following.len(),
            0,
            "Contract signer should not be following anyone at the beginning"
        );
        contract.follow(test_account_id.to_string());
        signer_following = contract.get_following(signer_account_id.clone());
        assert_eq!(
            signer_following.len(),
            1,
            "Contract signer should be following one person after execution of follow function"
        );
    }
}
