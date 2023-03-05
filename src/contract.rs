#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
// use cosmwasm_std:: ensure_eq;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use cw_utils::{ensure_from_older_version, nonpayable};

use crate::error::ContractError;
use crate::msg::{
    RegistrationResponse, ExecuteMsg, InstantiateMsg, ListRegistrationsResponse, MigrateMsg, QueryMsg,
    RecordMsg,
};
use crate::state::{last_registration, Config, Registration, CONFIG, REGISTRATIONS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:legends-registration";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// ## Description
/// Creates a new contract with the specified parameters in [`InstantiateMsg`].
/// This will set up the owner of the Registration Recrd contract
///
/// Returns a [`Response`] with the specified attributes if the operation was successful,
/// or a [`ContractError`] if the contract was not created.
/// ## Arguments
/// * `deps` - A [`DepsMut`] that contains the dependencies.
///
/// * `_env` - The [`Env`] of the blockchain.
///
/// * `_info` - The [`MessageInfo`] from the contract instantiator.
///
/// * `msg` - A [`InstantiateMsg`] which contains the parameters for creating the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = deps.api.addr_validate(&msg.owner)?;
    CONFIG.save(deps.storage, &Config { owner })?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner))
}

/// ## Description
/// Exposes all the execute functions available in the contract.
/// ## Arguments
/// * `deps` - A [`DepsMut`] that contains the dependencies.
///
/// * `env` - The [`Env`] of the blockchain.
///
/// * `info` - A [`MessageInfo`] that contains the message information.
///
/// * `msg` - The [`ExecuteMsg`] to run.
///
/// ## Execution Messages
/// * **ExecuteMsg::Record** Allow to store a registration.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Record(msg) => record(deps, env, info, msg),
    }
}

/// Write the registration if called by owner
pub fn record(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    record: RecordMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
// Write the registration if called by anyone 
    // let cfg = CONFIG.load(deps.storage)?;
    // ensure_eq!(info.sender, info.sender, ContractError::Unauthorized);

    // record.validate()?;

    // record this in the next available slot
    let id = last_registration(deps.as_ref())? + 1;
    let registration = Registration {
        created: env.block.time.seconds(),
        name: record.name.clone(),
        email: record.email,
        address: record.address,
    };
    REGISTRATIONS.save(deps.storage, id, &registration)?;

    Ok(Response::new()
        .add_attribute("method", "record")
        .add_attribute("name", record.name))
}

/// Query enumeration used to get an specific or all decisions
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Registration { id } => to_binary(&query_registration(deps, id)?),
        QueryMsg::ListRegistrations { start_after, limit } => {
            to_binary(&list_registrations(deps, start_after, limit)?)
        }
    }
}

fn query_registration(deps: Deps, id: u64) -> StdResult<RegistrationResponse> {
    Ok(REGISTRATIONS.load(deps.storage, id)?.into_response(id))
}

// settings for pagination
const MAX_LIMIT: u32 = 100;
const DEFAULT_LIMIT: u32 = 30;

fn list_registrations(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<ListRegistrationsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let registrations = REGISTRATIONS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (id, dec) = item?;
            Ok(dec.into_response(id))
        })
        .collect::<StdResult<Vec<_>>>()?;
    Ok(ListRegistrationsResponse { registrations })
}

/// Entry point for migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Timestamp;

    #[test]
    fn happy_path() {
        let mut deps = mock_dependencies();
        let owner = "the-man";

        // init
        let info = mock_info("someone", &[]);
        let msg = InstantiateMsg {
            owner: owner.to_string(),
        };
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // record something
        let record = RecordMsg {
            name: "My awesome registration".to_string(),
            email: "Let's all go to the beach and enjoy the sun!".to_string(),
            address: "osmo1clpqr4nrk4khgkxj78fcwwh6dl3uw4epasmvnj".to_string(),
        };
        let time1 = 111_222_333;
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(time1);
        let msg = ExecuteMsg::Record(record.clone());
        execute(deps.as_mut(), env, mock_info(owner, &[]), msg).unwrap();

        // record second registration
        let record2 = RecordMsg {
            name: "One more thing".to_string(),
            email: "John will bring a twelve pack for us all".to_string(),
            address: "osmo1clpqr4nrk4khgkxj78fcwwh6dl3uw4epasmvnj".to_string(),
        };
        let time2 = 111_444_555;
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(time2);
        let msg = ExecuteMsg::Record(record2.clone());
        execute(deps.as_mut(), env, mock_info(owner, &[]), msg).unwrap();

        // what do we expect?
        let expected1 = RegistrationResponse {
            id: 1,
            created: time1,
            name: record.name,
            email: record.email,
            address: record.address,
        };
        let expected2 = RegistrationResponse {
            id: 2,
            created: time2,
            name: record2.name,
            email: record2.email,
            address: record2.address,
        };

        let dec1 = query_registration(deps.as_ref(), 1).unwrap();
        assert_eq!(dec1, expected1);
        let dec2 = query_registration(deps.as_ref(), 2).unwrap();
        assert_eq!(dec2, expected2);

        let all = list_registrations(deps.as_ref(), None, None).unwrap();
        assert_eq!(all.registrations, vec![expected1, expected2]);
    }
}