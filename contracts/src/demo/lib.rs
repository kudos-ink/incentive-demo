#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;
pub mod types;

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod demo {
    use super::errors::DemoError;
    use super::types::ContributionId;
    use ink::storage::Mapping;
    use openbrush::{modifiers, traits::Storage};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Demo {
        // The field to save the owner of the contract
        #[storage_field]
        ownable: ownable::Data,

        // The approved `Contribution`.
        contributions: Mapping<ContributionId, AccountId>,
    }

    /// Emitted when a `contribution` is approved.
    #[ink(event)]
    pub struct ContributionApproval {
        id: ContributionId,
        contributor: AccountId,
    }

    impl Demo {
        /// Constructor that initializes an asset reward for a given workflow
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance
        }

        /// Approve contribution. This is triggered by a workflow run.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn approve(
            &mut self,
            contribution_id: ContributionId,
            contributor: AccountId,
        ) -> Result<(), DemoError> {
            match self.contributions.get(contribution_id) {
                Some(_) => Err(DemoError::ContributionAlreadyApproved),
                None => {
                    self.contributions.insert(contribution_id, &contributor);
                    self.env().emit_event(ContributionApproval {
                        id: contribution_id,
                        contributor,
                    });
                    Ok(())
                }
            }
        }

        /// Simply returns the `AccountId` of a given contribution.
        #[ink(message)]
        pub fn get_contributor(&self, contribution_id: ContributionId) -> Option<AccountId> {
            self.contributions.get(contribution_id)
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
            create_contract();
        }

        #[ink::test]
        fn approve_works() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let contribution_id = 1u64;

            set_next_caller(accounts.alice);
            assert_eq!(contract.approve(contribution_id, accounts.bob), Ok(()));

            // Validate `ContributionApproval` event emition
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());
            let decoded_events = decode_events(emitted_events);
            let Event::ContributionApproval(ContributionApproval { id, contributor }) =
                decoded_events[0];
            assert_eq!(id, contribution_id);
            assert_eq!(contributor, accounts.bob);

            let maybe_contributor = contract.contributions.get(contribution_id);
            assert_eq!(maybe_contributor, Some(accounts.bob));

            // Approve it again returns an error
            assert_eq!(
                contract.approve(contribution_id, accounts.alice),
                Err(DemoError::ContributionAlreadyApproved)
            );
        }

        #[ink::test]
        fn only_contract_owner_can_approve() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let contribution_id = 1u64;

            set_next_caller(accounts.bob);
            assert_eq!(
                contract.approve(contribution_id, accounts.alice),
                Err(DemoError::OwnableError(OwnableError::CallerIsNotOwner))
            );
        }

        #[ink::test]
        fn already_approved_contribution_fails() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let contribution_id = 1u64;

            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, accounts.alice);

            assert_eq!(
                contract.approve(contribution_id, accounts.alice),
                Err(DemoError::ContributionAlreadyApproved)
            );
        }

        #[ink::test]
        fn contributor_getter_works() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let contribution_id = 1u64;

            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, accounts.bob);

            assert_eq!(
                contract.get_contributor(contribution_id),
                Some(accounts.bob)
            );
            assert_eq!(contract.get_contributor(2u64), None);
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
        }

        /// Creates a new instance of `Demo`.
        ///
        /// Returns the `contract_instance`.
        fn create_contract() -> Demo {
            let accounts = default_accounts();
            set_next_caller(accounts.alice);
            Demo::new()
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
