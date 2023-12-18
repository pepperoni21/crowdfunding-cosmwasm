use std::collections::HashMap;

use cosmwasm_std::{entry_point, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use serde::{Deserialize, Serialize};

use crate::state::{ADMIN, PROJECTS, PROJECT_OWNERS};

use self::execute::{create_project, fund_project, remove_project};

mod execute;
mod query;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    ADMIN.save(deps.storage, &info.sender)?;

    PROJECTS.save(deps.storage, &HashMap::new())?;
    PROJECT_OWNERS.save(deps.storage, &vec![])?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QueryMsg {
    ListProjects {},
    GetProject { project_id: String },
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<cosmwasm_std::Binary> {
    match msg {
        QueryMsg::ListProjects {} => query::list_projects::list_projects(deps),
        QueryMsg::GetProject { project_id } => query::get_project::get_project(deps, project_id),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExecuteMsg {
    CreateProject {
        name: String,
        description: String,
        images: Option<Vec<String>>,
        funding_goal: u128,
        coin: String,
    },
    FundProject(String),
    RemoveProject(String),
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateProject {
            name,
            description,
            images,
            funding_goal,
            coin,
        } => create_project::create_project(
            deps,
            _env,
            info,
            create_project::CreateProjectMsg {
                name,
                description,
                images,
                funding_goal,
                coin,
            },
        ),
        ExecuteMsg::FundProject(project_id) => fund_project::fund_project(
            deps,
            _env,
            info,
            fund_project::FundProjectMsg { project_id },
        ),
        ExecuteMsg::RemoveProject(project_id) => remove_project::remove_project(
            deps,
            _env,
            info,
            remove_project::RemoveProjectMsg { project_id },
        ),
    }
}
