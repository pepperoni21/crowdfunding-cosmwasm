use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Empty,
};

use crate::{
    contract::{execute, instantiate, ExecuteMsg},
    project::Project,
    state::PROJECTS,
};

#[test]
fn test_fund_project() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let message_info = mock_info(
        "cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee",
        &coins(10, "uusd".to_string()),
    );

    let res = instantiate(deps.as_mut(), env.clone(), message_info.clone(), Empty {}).unwrap();
    dbg!(res);

    let mut projects = PROJECTS.load(deps.as_ref().storage).unwrap();
    projects.insert(
        "test-project".into(),
        Project::new(
            Addr::unchecked("cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee"),
            "test-project".into(),
            "Test Project".into(),
            "Test description".into(),
            vec![],
            300,
            "uusd".into(),
        ),
    );
    PROJECTS.save(deps.as_mut().storage, &projects).unwrap();

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        ExecuteMsg::FundProject("test-project".to_string()),
    )
    .unwrap();
    dbg!(&res);

    let projects = PROJECTS.load(deps.as_ref().storage).unwrap();
    let project = projects.get("test-project").unwrap();
    dbg!(project);
    assert_eq!(project.total_funding, 10);

    let message_info = mock_info(
        "cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee",
        &coins(10, "other-token".to_string()),
    );
    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        ExecuteMsg::FundProject("test-project".to_string()),
    );
    dbg!(&res);
    assert!(res.is_err());
}
