#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;
pub mod types;

#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod demo {
    use super::errors::DemoError;
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use openbrush::{modifiers, traits::Storage};
    use super::types::{ContributorId, ContributionId};
    use super::types::Contribution;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Demo {
        #[storage_field]
        ownable: ownable::Data,

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

    impl Demo {
        /// Constructor that initializes an asset reward for a given workflow
        #[ink(constructor)]
        pub fn new() -> Self {
 
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            instance
        }

        /// Register the caller as an aspiring contributor.
        #[ink(message)]
        pub fn register_identity(&mut self, id: String) -> Result<(), DemoError> {
            if self.identity_is_known(id.clone()) {
                return Err(DemoError::IdentityAlreadyRegistered);
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
        ) -> Result<(), DemoError> {
            let contributor = self.identities.get(contributor_id).ok_or(DemoError::UnknownContributor)?;
            
            match self.contributions.get(contribution_id) {
                Some(contribution) => {
                    if contribution.claimed {
                        Err(DemoError::ContributionAlreadyClaimed)
                    } else {
                        Err(DemoError::ContributionAlreadyApproved)
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
        
        #[ink(message)]
        pub fn check(&self, contribution_id: ContributorId) -> Result<bool, DemoError>{
            let contribution = self.contributions.get(contribution_id).ok_or(DemoError::NoContributionApprovedYet)?;
            Ok(contribution.contributor == Self::env().caller())
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
            create_contract();
        }

        #[ink::test]
        fn register_identity_works() {
            let accounts = default_accounts();
            let mut contract = create_contract();
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
            let mut contract = create_contract();
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());
            assert_eq!(
                contract.register_identity(identity.to_string()),
                Err(DemoError::IdentityAlreadyRegistered)
            );
        }

        #[ink::test]
        fn approve_works() {
            let accounts = default_accounts();
            let mut contract = create_contract();
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
                Err(DemoError::ContributionAlreadyApproved)
            );
        }

        #[ink::test]
        fn only_contract_owner_can_approve() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(DemoError::OwnableError(OwnableError::CallerIsNotOwner))
            );
        }

        #[ink::test]
        fn already_approved_contribution_fails() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let identity = "bobby";
            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());

            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(DemoError::ContributionAlreadyApproved)
            );
        }

        #[ink::test]
        fn approve_unknown_contributor_identity_fails() {
            let mut contract = create_contract();
            let identity = "unknown";
            let contribution_id = 1u64;
            assert_eq!(
                contract.approve(contribution_id, identity.to_string()),
                Err(DemoError::UnknownContributor)
            );
        }

        #[ink::test]
        fn check_works() {
            let accounts = default_accounts();
            let mut contract = create_contract();
            let identity = "bobby";
            let identity2 = "charlie";

            set_next_caller(accounts.bob);
            let _ = contract.register_identity(identity.to_string());

            set_next_caller(accounts.charlie);
            let _ = contract.register_identity(identity2.to_string());

            let contribution_id = 1u64;
            set_next_caller(accounts.alice);
            let _ = contract.approve(contribution_id, identity.to_string());
            
            set_next_caller(accounts.bob);
            assert_eq!(
                contract.check(contribution_id),
                Ok(true)
            );

            set_next_caller(accounts.charlie);
            assert_eq!(
                contract.check(contribution_id),
                Ok(false)
            );
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
