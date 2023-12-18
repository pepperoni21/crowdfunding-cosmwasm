use cosmwasm_std::{Binary, Deps, StdResult};

use crate::{error::CrowdfundingError, project::Project, state::PROJECTS};

pub fn list_projects(deps: Deps) -> StdResult<Binary> {
    let projects = PROJECTS.load(deps.storage)?;
    let projects = projects.into_iter().map(|(_, project)| project).collect::<Vec<Project>>();
    Ok(Binary::from(
        serde_json::to_string(&projects)
            .map_err(|err| CrowdfundingError::SerializationError(err))?
            .as_bytes(),
    ))
}
