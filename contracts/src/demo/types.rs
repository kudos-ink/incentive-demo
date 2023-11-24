pub use ink::prelude::string::String;
use ink::primitives::AccountId;

pub type ContributionId = u64;
pub type ContributorId = u64;


/// A Contribution is represented by:
/// - a unique id.
/// - the contributor; allowed to claim the reward.
#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
)]
pub struct Contribution {
    // The unique contribution ID (e.g. the Github issue #id).
    pub id: ContributionId,
    // The contributor public key (e.g. extract from the `identities` mapping).
    pub contributor: AccountId,
    pub claimed: bool,
}