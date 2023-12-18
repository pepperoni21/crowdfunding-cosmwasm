use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Empty,
};

use crate::{
    contract::{execute, instantiate, ExecuteMsg},
    project::Project,
    state::PROJECTS,
};

#[test]
fn test_remove_project() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let message_info = mock_info("cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee", &[]);

    let _ = instantiate(deps.as_mut(), env.clone(), message_info.clone(), Empty {}).unwrap();

    let mut projects = PROJECTS.load(deps.as_ref().storage).unwrap();
    projects.insert(
        "test-project".into(),
        Project::new(
            message_info.sender.clone(),
            "test-project".into(),
            "Test Project".into(),
            "Test Description".into(),
            vec![],
            1000,
            "uusd".into(),
        ),
    );
    PROJECTS.save(deps.as_mut().storage, &projects).unwrap();

    let other_message_info = mock_info("cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtef", &[]);
    let res = execute(
        deps.as_mut(),
        env.clone(),
        other_message_info.clone(),
        ExecuteMsg::RemoveProject("test-project".to_string()),
    );
    dbg!(&res);
    assert!(res.is_err());

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        ExecuteMsg::RemoveProject("test-project".to_string()),
    )
    .unwrap();
    dbg!(&res);
    assert_eq!(0, res.messages.len());

    let res = execute(
        deps.as_mut(),
        env.clone(),
        message_info.clone(),
        ExecuteMsg::RemoveProject("test-project".to_string()),
    );
    dbg!(&res);
    assert!(res.is_err());
}
