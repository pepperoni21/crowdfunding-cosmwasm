use cosmwasm_std::{coins, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    error::CrowdfundingError,
    state::{ADMIN, PROJECTS},
};

pub struct FundProjectMsg {
    pub project_id: String,
}

impl From<String> for FundProjectMsg {
    fn from(val: String) -> Self {
        FundProjectMsg { project_id: val }
    }
}

pub fn fund_project(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: FundProjectMsg,
) -> StdResult<Response> {
    let mut projects = PROJECTS.load(deps.storage)?;

    if !projects.contains_key(&msg.project_id) {
        return Err(CrowdfundingError::ProjectNotFound.into());
    }

    let project = projects.get_mut(&msg.project_id).unwrap();

    if info.funds.len() != 1 {
        return Err(CrowdfundingError::InvalidFundsAmount.into());
    }

    let coin: &Coin = &info.funds[0].clone();
    if coin.denom != project.coin {
        return Err(CrowdfundingError::InvalidFundsAmount.into());
    }

    let amount = coin.amount.u128();

    project.total_funding += amount;
    project.add_history_item(amount, info.sender, env.block.time);

    let project_owner = project.owner.clone();

    PROJECTS.save(deps.storage, &projects)?;

    let admin_fee = amount / 10;
    let messages = vec![
        BankMsg::Send {
            to_address: ADMIN.load(deps.storage).unwrap().into_string(),
            amount: coins(admin_fee, coin.denom.clone()),
        },
        BankMsg::Send {
            to_address: project_owner.into_string(),
            amount: coins(amount - admin_fee, coin.denom.clone()),
        },
    ];

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "fund_project")
        .add_attribute("project_id", msg.project_id)
        .add_attribute("amount", amount.to_string())
        .add_attribute("coin", coin.denom.clone()))
}
