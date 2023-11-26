use openbrush::contracts::traits::ownable::OwnableError;

/// Errors that can occur upon calling this contract.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum DemoError {
    OwnableError(OwnableError),
    /// An aspiring contributor identity is already registered in the DB.
    IdentityAlreadyRegistered,
    /// A `contribution` is already approved in the DB.
    ContributionAlreadyApproved,
    /// No `contribution` is approved yet in the DB.
    NoContributionApprovedYet,
}

impl From<OwnableError> for DemoError {
    fn from(error: OwnableError) -> Self {
        DemoError::OwnableError(error)
    }
}
