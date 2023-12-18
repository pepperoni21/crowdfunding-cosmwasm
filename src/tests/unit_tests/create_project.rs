use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Empty,
};

use crate::{
    contract::{execute, instantiate, ExecuteMsg},
    state::{PROJECTS, PROJECT_OWNERS},
};

#[test]
fn test_create_project() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let message_info = mock_info("cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee", &[]);

    let res = instantiate(deps.as_mut(), env.clone(), message_info.clone(), Empty {}).unwrap();
    dbg!(res);

    let mut create_project_msg = ExecuteMsg::CreateProject {
        name: "Test Project".to_string(),
        description: "Test Description".to_string(),
        funding_goal: 1000000,
        images: None,
        coin: "uusd".to_string(),
    };

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        create_project_msg.clone(),
    )
    .unwrap();
    assert_eq!(0, res.messages.len());
    let projects = PROJECTS.load(deps.as_ref().storage).unwrap();
    dbg!(&projects);
    let project_owners = PROJECT_OWNERS.load(deps.as_ref().storage).unwrap();
    dbg!(&project_owners);

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        create_project_msg.clone(),
    );
    dbg!(&res);
    assert!(res.is_err());

    create_project_msg = ExecuteMsg::CreateProject {
        name: "Test Project 2".to_string(),
        description: "Test Description 2".to_string(),
        funding_goal: 1000000,
        images: None,
        coin: "uusd".to_string(),
    };

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        create_project_msg,
    );
    dbg!(&res);
    assert!(res.is_err());

    create_project_msg = ExecuteMsg::CreateProject {
        name: "Test Project 3".to_string(),
        description: "Test Description 3".to_string(),
        funding_goal: 1000000,
        images: None,
        coin: "uusd".to_string(),
    };

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        create_project_msg,
    );
    dbg!(&res);
    assert!(res.is_err());
}
