mod contract_tests;
mod execute;
mod query;
pub mod state;

pub use crate::state::Cw1155Contract;
use cosmwasm_std::Empty;

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:cw1155-base";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw1155::error::Cw1155ContractError;
    use cw1155::execute::Cw1155Execute;
    use cw1155::msg::{Cw1155ExecuteMsg, Cw1155InstantiateMsg, Cw1155QueryMsg};
    use cw1155::query::Cw1155Query;
    use cw721::state::DefaultOptionMetadataExtension;

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155InstantiateMsg,
    ) -> Result<Response Cw1155ContractError> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let tract = Cw1155Contract::<DefaultOptionMetadataExtension, Empty, Empty>::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155ExecuteMsg<DefaultOptionMetadataExtension, Empty>,
    ) -> Result<Response, Cw1155ContractError> {
        let tract = Cw1155Contract::<DefaultOptionMetadataExtension, Empty, Empty>::default();
        tract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: Cw1155QueryMsg<DefaultOptionMetadataExtension>) -> StdResult<Binary> {
        let tract = Cw1155Contract::<DefaultOptionMetadataExtension, Empty, Empty>::default();
        tract.query(deps, env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, Cw1155ContractError> {
        // make sure the correct contract is being upgraded, and it's being
        // upgraded from the correct version.
        cw2::assert_contract_version(deps.as_ref().storage, CONTRACT_NAME, EXPECTED_FROM_VERSION)?;

        // update contract version
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default())
    }
}
