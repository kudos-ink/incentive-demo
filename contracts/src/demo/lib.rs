#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod traits;

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod demo {
    use super::traits::types::{Contribution, ContributionId, ContributorId, String};
    use super::traits::workflow::{WorkflowError, *};
    use ink::storage::Mapping;
    use openbrush::{modifiers, traits::Storage};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Demo {
        #[storage_field]
        ownable: ownable::Data,

        // The contribution reward amount.
        reward: Balance,

        // The approved `Contribution`.
        contributions: Mapping<ContributionId, Contribution>,

        // The registered contributors ids database.
        // The key refers to a registered and unique contribution ID (e.g. the Github issue #id).
        // The value is the associated registered `AccountId` (public key) of the contributor.
        identities: Mapping<String, AccountId>, // String refers to the contributo id (e.g. github ID)
    }

    /// Emitted when an `id` is registered by an aspiring contributor.
    #[ink(event)]
    pub struct IdentityRegistered {
        id: String,
        caller: AccountId,
    }

    /// Emitted when a `contribution` is approved.
    #[ink(event)]
    pub struct ContributionApproval {
        id: ContributorId,
        contributor: AccountId,
    }

    /// Emitted when the reward associated with the `contribution` is claimed.
    #[ink(event)]
    pub struct RewardClaimed {
        contribution_id: ContributorId,
        contributor: AccountId,
        reward: Balance,
    }

    impl Workflow for Demo {
        /// Register the caller as an aspiring contributor.
        ///
        /// Constraint(s):
        /// 1. The `id` id should not already be registered.
        ///
        /// A `IdentityRegistered` event is emitted.
        #[ink(message)]
        fn register_identity(&mut self, id: String) -> Result<(), WorkflowError> {
            self.register_identity(id)
        }

        /// Approve contribution. This is triggered by a workflow run.
        ///
        /// Constraint(s):
        /// 1. The `contribution_id` should not already be approved.
        /// 2. The `contributor_id` must be registered.
        ///
        /// An `ContributionApproval` event is emitted.
        #[ink(message)]
        #[modifiers(only_owner)]
        fn approve(
            &mut self,
            contribution_id: ContributorId,
            contributor_id: String,
        ) -> Result<(), WorkflowError> {
            self.approve(contribution_id, contributor_id)
        }

        /// Check the ability to claim for a given `contribution_id`.
        ///
        /// Constraint(s):
        /// 1. A `contribution` must be approved.
        /// 2. The `contribution_id` must be the same as the one in the approved `contribution`.
        /// 3. The caller has to be the contributor of the approved `contribution`.
        /// 4. The claim must be available (marked as false in the claims mapping).
        #[ink(message)]
        fn can_claim(&self, contribution_id: ContributorId) -> Result<(), WorkflowError> {
            self.can_claim(contribution_id)
        }

        /// Claim reward for a given `contribution_id`.
        ///
        /// Constraint(s): Ensure `can_claim`.
        ///
        /// A `RewardClaimed` event is emitted.
        #[ink(message)]
        fn claim(&mut self, contribution_id: ContributorId) -> Result<(), WorkflowError> {
            self.claim(contribution_id)
        }

        #[ink(message)]
        fn check(&self, contribution_id: u64) -> Result<bool, WorkflowError>{
            self.check(contribution_id)
        }
    }

    impl Demo {
        /// Constructor that initializes an asset reward for a given workflow
        #[ink(constructor, payable)]
        pub fn new(reward: Balance) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            ownable::Internal::_init_with_owner(&mut instance, caller);
            Self {
                reward,
                ..instance
            }
        }

        /// Register the caller as an aspiring contributor.
        #[ink(message)]
        pub fn register_identity(&mut self, id: String) -> Result<(), WorkflowError> {
            if self.identity_is_known(id.clone()) {
                return Err(WorkflowError::IdentityAlreadyRegistered);
            }

            let caller = Self::env().caller();
            self.identities.insert(id.clone(), &caller);

            self.env().emit_event(IdentityRegistered { id, caller });

            Ok(())
        }

        /// Approve contribution. This is triggered by a workflow run.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn approve(
            &mut self,
            contribution_id: ContributorId,
            contributor_id: String,
        ) -> Result<(), WorkflowError> {
            let contributor = self.identities.get(contributor_id).ok_or(WorkflowError::UnknownContributor)?;
            
            match self.contributions.get(contribution_id) {
                Some(contribution) => {
                    if contribution.claimed {
                        Err(WorkflowError::ContributionAlreadyClaimed)
                    } else {
                        Err(WorkflowError::ContributionAlreadyApproved)
                    }
                },
                None => {
                    let contribution = Contribution {
                        claimed: false,
                        id: contribution_id,
                        contributor

                    };
                    self.contributions.insert(contribution_id, &contribution);

                    self.env().emit_event(ContributionApproval {
                        id: contribution_id,
                        contributor,
                    });

                    Ok(())
                }
            }
        }

        /// Check the ability to claim for a given `contribution_id`.
        #[ink(message)]
        pub fn can_claim(&self, contribution_id: ContributorId) -> Result<(), WorkflowError> {
            self.ensure_can_claim(contribution_id)?;

            Ok(())
        }

        /// Claim reward for a given `contribution_id`.
        #[ink(message)]
        pub fn claim(&mut self, contribution_id: ContributorId) -> Result<(), WorkflowError> {
            let mut contribution = self.ensure_can_claim(contribution_id)?;

            // Perform the reward claim
            self.env().transfer(contribution.contributor, self.reward).map_err(|_err| WorkflowError::PaymentFailed)?;

            contribution.claimed = true;
            self.contributions.insert(contribution_id, &contribution);

            self.env().emit_event(RewardClaimed {
                contribution_id,
                contributor: contribution.contributor,
                reward: self.reward,
            });

            Ok(())
        }
        
        #[ink(message)]
        pub fn check(&self, contribution_id: ContributorId) -> Result<bool, WorkflowError>{
            let contribution = self.contributions.get(contribution_id).ok_or(WorkflowError::NoContributionApprovedYet)?;
            Ok(contribution.contributor == Self::env().caller())
        }
        /// Simply returns the reward amount.
        #[ink(message)]
        pub fn get_reward(&self) -> Balance {
            self.reward
        }


        /// A helper function to ensure a contributor can claim the reward.
        pub fn ensure_can_claim(
            &self,
            contribution_id: ContributorId,
        ) -> Result<Contribution, WorkflowError> {
            // Check if a contribution is set
            let contribution = self.contributions.get(contribution_id).ok_or(WorkflowError::NoContributionApprovedYet)?;

            // Verify the contribution ID
            if contribution_id != contribution.id {
                return Err(WorkflowError::UnknownContribution);
            }

            // Verify the caller is the contributor
            if Self::env().caller() != contribution.contributor {
                return Err(WorkflowError::CallerIsNotContributor);
            }

            // Check if the reward has already been claimed
            if contribution.claimed {
                return Err(WorkflowError::ContributionAlreadyClaimed);
            }

            Ok(contribution)
        }

        /// A helper function to detect whether an aspiring contributor id has been registered in the storage.
        pub fn identity_is_known(&self, id: String) -> bool {
            self.identities.get(id).is_some()
        }

    }


    #[cfg(test)]
    mod tests {
        /// Accounts
        /// ALICE -> contract owner
        /// BOB -> contributor

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink::env::test::EmittedEvent;
        type Event = <Demo as ::ink::reflect::ContractEventBase>::Type;

        /// We test if the constructor does its job.
        #[ink::test]
        fn new_works() {
            let reward = 10u128;
            let contract = create_contract(reward);
            assert_eq!(contract.get_reward(), reward);
            assert_eq!(get_balance(contract_id()), reward);
        }

        #[ink::test]
        fn register_identity_works() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let bob_identity = "bobby";
            set_next_caller(accounts.bob);
            assert_eq!(
                contract.register_identity(bob_identity.to_string()),
                Ok(())
            );

            // Validate `IdentityRegistered` event emition
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());
            let decoded_events = decode_events(emitted_events);
            if let Event::IdentityRegistered(IdentityRegistered { caller, id }) = &decoded_events[0] {
                assert_eq!(id, bob_identity);
                assert_eq!(caller, &accounts.bob);
            } else {
                panic!("encountered unexpected event kind: expected a IdentityRegistered event")
            }
            
            let maybe_account = contract.identities.get(bob_identity.to_string());
            assert_eq!(
                maybe_account,
                Some(accounts.bob)
            );
        }

        #[ink::test]
        fn already_registered_identity_fails() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());
            assert_eq!(
                contract.register_identity(identity.to_string()),
                Err(WorkflowError::IdentityAlreadyRegistered)
            );
        }

        #[ink::test]
        fn approve_works() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Ok(())
            );

            // Validate `ContributionApproval` event emition
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(2, emitted_events.len());
            let decoded_events = decode_events(emitted_events);
            if let Event::ContributionApproval(ContributionApproval { id, contributor }) = decoded_events[1] {
                assert_eq!(id, contribution_id);
                assert_eq!(contributor, accounts.bob);
            } else {
                panic!("encountered unexpected event kind: expected a ContributionApproval event")
            }

            let maybe_contribution = contract.contributions.get(contribution_id);
            assert_eq!(
                maybe_contribution,
                Some(Contribution {id: contribution_id, contributor: accounts.bob, claimed: false})
            );
            
            // Approve it again returns an error
            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(WorkflowError::ContributionAlreadyApproved)
            );
        }

        #[ink::test]
        fn only_contract_owner_can_approve() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(WorkflowError::OwnableError(OwnableError::CallerIsNotOwner))
            );
        }

        #[ink::test]
        fn already_approved_contribution_fails() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());

            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(WorkflowError::ContributionAlreadyApproved)
            );
        }

        #[ink::test]
        fn approve_unknown_contributor_identity_fails() {
            let mut contract = create_contract(1u128);
            let identity = "unknown";
            let contribution_id = 1u64;
            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(WorkflowError::UnknownContributor)
            );
        }

        #[ink::test]
        fn can_claim_works() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());
            
            set_next_caller(accounts.bob);
            assert_eq!(
                contract.can_claim(contribution_id),
                Ok(())
            );
        }

        #[ink::test]
        fn claim_works() {
            let accounts = default_accounts();
            let single_reward = 1u128;
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());
            
            let bob_initial_balance = get_balance(accounts.bob);
            set_next_caller(accounts.bob);
            assert_eq!(contract.claim(contribution_id), Ok(()));
            assert_eq!(
                get_balance(accounts.bob),
                bob_initial_balance + contract.reward
            );

            let maybe_contribution = contract.contributions.get(contribution_id);
            assert_eq!(
                maybe_contribution,
                Some(Contribution {id: contribution_id, contributor: accounts.bob, claimed: true})
            );

            // Validate `RewardClaimed` event emition
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(3, emitted_events.len());
            let decoded_events = decode_events(emitted_events);
            if let Event::RewardClaimed(RewardClaimed { contribution_id, contributor, reward }) = decoded_events[2] {
                assert_eq!(contribution_id, contribution_id);
                assert_eq!(contributor, accounts.bob);
                assert_eq!(reward, single_reward);
            } else {
                panic!("encountered unexpected event kind: expected a RewardClaimed event")
            }
        }

        #[ink::test]
        fn cannot_claim_non_approved_contribution() {
            let accounts = default_accounts();
            let contract = create_contract(1u128);
            set_next_caller(accounts.bob);

            let contribution_id = 1u64;
            assert_eq!(
                contract.can_claim(contribution_id),
                Err(WorkflowError::NoContributionApprovedYet)
            );
        }


        #[ink::test]
        fn cannot_claim_if_not_contributor() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.eve);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());

            set_next_caller(accounts.bob);
            assert_eq!(
                contract.can_claim(contribution_id),
                Err(WorkflowError::CallerIsNotContributor)
            );
        }

        #[ink::test]
        fn cannot_claim_already_claimed_reward() {
            let accounts = default_accounts();
            let mut contract = create_contract(1u128);
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());

            set_next_caller(accounts.bob);
            let _ = contract.claim(contribution_id);
            assert_eq!(
                contract.can_claim(contribution_id),
                Err(WorkflowError::ContributionAlreadyClaimed)
            );
        }


        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn contract_id() -> AccountId {
            ink::env::test::callee::<ink::env::DefaultEnvironment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }

        fn get_balance(account: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account)
                .expect("Cannot get account balance")
        }

        /// Creates a new instance of `Demo`.
        ///
        /// Returns the `contract_instance`.
        fn create_contract(reward: Balance) -> Demo {
            let accounts = default_accounts();
            set_next_caller(accounts.alice);
            set_balance(contract_id(), reward);
            Demo::new(reward)
        }

        fn decode_events(emittend_events: Vec<EmittedEvent>) -> Vec<Event> {
            emittend_events
                .into_iter()
                .map(|event| {
                    <Event as scale::Decode>::decode(&mut &event.data[..]).expect("invalid data")
                })
                .collect()
        }
    }

}
