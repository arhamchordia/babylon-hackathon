use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use crate::{
    contract::{instantiate, query_destinations},
    msg::InstantiateMsg,
    tests::setup::{DESTINATION_IDS, OWNER, USER},
};

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(USER, &[]);

    let result = instantiate(
        deps.as_mut(),
        env.clone(),
        info,
        InstantiateMsg {
            owner: OWNER.to_string(),
            destinations: DESTINATION_IDS.iter().map(|&s| s.to_string()).collect(), // this is really ugly and should be done simpler,
        },
    );
    assert!(result.is_ok());

    let response = query_destinations(deps.as_ref()).unwrap();
    assert_eq!(response.destinations, DESTINATION_IDS);
}
