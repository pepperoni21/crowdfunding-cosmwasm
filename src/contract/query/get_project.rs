use cosmwasm_std::{Binary, Deps, StdResult};

use crate::{error::CrowdfundingError, state::PROJECTS};

pub fn get_project(deps: Deps, project_id: String) -> StdResult<Binary> {
    let projects = PROJECTS.load(deps.storage)?;
    match projects.get(&project_id) {
        Some(project) => Ok(Binary::from(
            serde_json::to_string(&project)
                .map_err(|err| CrowdfundingError::SerializationError(err))?
                .as_bytes(),
        )),
        None => Err(CrowdfundingError::ProjectNotFound.into()),
    }
}
