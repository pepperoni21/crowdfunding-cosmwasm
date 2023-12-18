use cosmwasm_std::{coins, Empty};
use cw_multi_test::{App, BankSudo, ContractWrapper, Executor, SudoMsg};

use crate::contract::{execute, instantiate, query, ExecuteMsg};

#[test]
fn integration_test() {
    let mut app = App::default();

    // Create an address for the contract owner
    let owner = app.api().addr_make("owner");

    // Create an address for the user
    let user = app.api().addr_make("user");
    // Mint 40 uusd to the user
    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: user.clone().to_string(),
        amount: coins(40, "uusd"),
    }))
    .unwrap();

    // Deploy the contract to the fake chain
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract
    let addr = app
        .instantiate_contract(code_id, owner.clone(), &Empty {}, &[], "Crowdfunding", None)
        .unwrap();

    // Create a project
    let create_project_msg = ExecuteMsg::CreateProject {
        name: "My project".to_string(),
        description: "This is my project".to_string(),
        images: None,
        funding_goal: 1000,
        coin: "uusd".to_string(),
    };
    let resp = app.execute_contract(owner.clone(), addr.clone(), &create_project_msg, &[]);
    dbg!(&resp);
    assert!(resp.is_ok());

    // Fund the project as the user with 40 uusd
    let fund_project_msg = ExecuteMsg::FundProject("my-project".to_string());
    let resp = app.execute_contract(
        user.clone(),
        addr.clone(),
        &fund_project_msg,
        &coins(40, "uusd"),
    );
    dbg!(&resp);
    assert!(resp.is_ok());
    // Fund the project as the user, it should fail this time because the user doesn't have enough funds
    let resp = app.execute_contract(
        user.clone(),
        addr.clone(),
        &fund_project_msg,
        &coins(40, "uusd"),
    );
    dbg!(&resp);
    assert!(resp.is_err());
}
