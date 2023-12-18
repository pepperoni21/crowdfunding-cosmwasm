use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info}, Empty};

use crate::{contract::{instantiate, ExecuteMsg, execute, query, QueryMsg}, project::Project};

#[test]
fn test_get_project() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let message_info = mock_info("cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee", &[]);

    let res = instantiate(deps.as_mut(), env.clone(), message_info.clone(), Empty {}).unwrap();
    dbg!(res);

    let create_project_msg = ExecuteMsg::CreateProject {
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
    let project_id = res.attributes[1].value.clone();

    let res = query(deps.as_ref(), env.clone(), QueryMsg::GetProject { project_id: project_id.clone() }).unwrap();
    let project = Project::from_slice(&res).unwrap();
    dbg!(&project);

    assert_eq!(project_id, project.id);
}