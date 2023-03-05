use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::RegistrationResponse;
use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_storage_plus::{Item, Map};

/// Configuration Item
pub const CONFIG: Item<Config> = Item::new("config");

/// Desicion Map <Registration ID, Registration>
pub const REGISTRATIONS: Map<u64, Registration> = Map::new("registrations");

/// Configuration
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    /// contract owner, wynd foundation
    pub owner: Addr,
}

/// Registration
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Registration {
    /// Creation time as unix time stamp (in seconds)
    pub created: u64,
    /// name of the registration
    pub name: String,
    /// Email address of the registration
    pub email: String,
    /// Wallet address of the registration
    pub address: String,
}

impl Registration {
    /// ## Description
    /// Return a [`RegistrationResponse`] from [`Registration`].
    ///
    /// Returns a new object [`RegistrationResponse`].
    /// ## Arguments
    /// * `id` - unique id that index a Registration.
    pub fn into_response(self, id: u64) -> RegistrationResponse {
        RegistrationResponse {
            id,
            created: self.created,
            name: self.name,
            email: self.email,
            address: self.address,
        }
    }
}

/// Returns the last recorded registration id (auto-incremented count)
pub fn last_registration(deps: Deps) -> StdResult<u64> {
    REGISTRATIONS
        .keys(deps.storage, None, None, Order::Descending)
        .next()
        .unwrap_or(Ok(0))
}