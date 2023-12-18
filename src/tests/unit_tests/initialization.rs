use crate::{contract::instantiate, state::ADMIN};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Empty,
};

#[test]
fn test_initialization() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let message_info = mock_info("cosmos1ppgxtwytwyszc7xj6cu2as7jpcaqnwwgjqjtee", &[]);

    let res = instantiate(deps.as_mut(), env.clone(), message_info.clone(), Empty {}).unwrap();

    assert_eq!(0, res.messages.len());

    let admin = ADMIN.load(deps.as_ref().storage).unwrap();
    assert_eq!(message_info.sender, admin);
}
