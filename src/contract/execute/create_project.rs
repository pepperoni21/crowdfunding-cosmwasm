use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use serde::{Deserialize, Serialize};

use crate::{
    project::Project,
    state::{PROJECTS, PROJECT_OWNERS},
};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CreateProjectMsg {
    pub name: String,
    pub description: String,
    pub images: Option<Vec<String>>,
    pub funding_goal: u128,
    pub coin: String,
}

pub fn create_project(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: CreateProjectMsg,
) -> StdResult<Response> {
    let sender = info.sender;

    let mut project_owners = PROJECT_OWNERS.load(deps.storage)?;

    let id = crate::utils::title_to_id(&msg.name);

    let mut projects = crate::state::PROJECTS.load(deps.storage)?;

    if project_owners.contains(&sender) {
        return Err(StdError::generic_err("You already own a project"));
    }

    if projects.contains_key(&id) {
        return Err(StdError::generic_err("Project already exists"));
    }

    let funding_goal = msg.funding_goal;

    if funding_goal == 0 {
        return Err(StdError::generic_err("Funding goal must be greater than 0"));
    }

    let project = Project::new(
        sender.clone(),
        id.clone(),
        msg.name,
        msg.description,
        msg.images.unwrap_or_default(),
        funding_goal,
        msg.coin,
    );
    projects.insert(id.clone(), project);
    PROJECTS.save(deps.storage, &projects)?;

    project_owners.push(sender);
    PROJECT_OWNERS.save(deps.storage, &project_owners)?;

    Ok(Response::new()
        .add_attribute("action", "create_project")
        .add_attribute("project_id", id))
}
