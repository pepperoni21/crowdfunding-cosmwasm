use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{error::CrowdfundingError, state::PROJECTS};

pub struct RemoveProjectMsg {
    pub project_id: String,
}

impl From<String> for RemoveProjectMsg {
    fn from(val: String) -> Self {
        RemoveProjectMsg { project_id: val }
    }
}

pub fn remove_project(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RemoveProjectMsg,
) -> StdResult<Response> {
    let mut projects = PROJECTS.load(deps.storage)?;

    if projects.get(&msg.project_id).is_none() {
        return Err(CrowdfundingError::ProjectNotFound.into());
    }

    if projects.get(&msg.project_id).unwrap().owner != info.sender {
        return Err(CrowdfundingError::Unauthorized.into());
    }

    projects.remove(&msg.project_id);
    PROJECTS.save(deps.storage, &projects)?;

    Ok(Response::new()
        .add_attribute("action", "remove_project")
        .add_attribute("project_id", msg.project_id))
}
