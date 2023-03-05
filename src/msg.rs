use crate::error::ContractError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Initialization message that only setup an owner
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The address who can add registrations to the log
    pub owner: String,
}

/// Execute message enumeration
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Store a Registration
    Record(RecordMsg),
}

/// Represents a Registration track
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct RecordMsg {
    /// Title of the registration
    pub name: String,
    /// Email address of the registration
    pub email: String,
    /// Wallet address of the registration
    pub address: String,
}


impl RecordMsg {
    /// ## Description
    /// Sanity check of received [`RecordMsg`].
    /// This will check if interna fields are valid
    /// Returns a [`Empty`] on successful,
    /// or a [`ContractError`] if the contract was not created.
    /// # Examples
    ///
    /// ```rust
    /// use legends_registration::msg::RecordMsg;
    /// use legends_registration::error::ContractError;
    /// let record: RecordMsg = RecordMsg {
    ///     name: String::from("name"),
    ///     email: String::from("email"),
    ///     address: String::from("address"),
    ///   
    ///    
    /// };
    /// let error: ContractError = record.validate().unwrap_err();
    /// println!("{}",error.to_string());
    /// assert!(error.to_string() == String::from("Body must be between 20 and 9192 characters"));
    /// ```
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.name.len() < 4 || self.name.len() > 128 {
            return Err(ContractError::InvalidLength("Title", 4, 128));
        }
        if self.email.len() < 20 || self.name.len() > 9192 {
            return Err(ContractError::InvalidLength("Body", 20, 9192));
        }
        Ok(())
    }
}

/// Query input Message enumeration
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query for an specific Registration represented by an ID
    Registration {
        /// Registration ID
        id: u64,
    },
    /// Query all Registration makes using pagination as optional
    ListRegistrations {
        /// ID to start from. If None, it will start from 1
        start_after: Option<u64>,
        /// Represents how many rows will return the [`RegistrationResponse`]
        limit: Option<u32>,
    },
}

/// Registration Response that may contain the public IPFS link or private hash for the document
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema, Debug)]
pub struct RegistrationResponse {
    /// Registration UID
    pub id: u64,
    /// Creation time as unix time stamp (in seconds)
    pub created: u64,
    /// Title of the registration
    pub name: String,
    /// Email address of the registration
    pub email: String,
    /// Wallet address of the registration
    pub address: String,
}

/// Registration Response list wrapper
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema, Debug)]
pub struct ListRegistrationsResponse {
    /// Registration Response list
    pub registrations: Vec<RegistrationResponse>,
}

/// Message that is passed during migration
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct MigrateMsg {}