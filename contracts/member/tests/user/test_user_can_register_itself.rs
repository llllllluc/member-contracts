use cosmwasm_std::{Uint128, Uint64};

use member_pkg::{
    msg::{QueryMsg, QueryUserByAddrMsg, UserResponse},
    user::User,
};

use crate::helpers::{proper_instantiate, register_user};

#[test]
fn test_user_can_register_itself() {
    let (mut app, cw_member_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();
    register_user(&mut app, &cw_member_contract_addr, &user_1_addr).unwrap();
    let query_user_1_res: UserResponse = app
        .wrap()
        .query_wasm_smart(
            cw_member_contract_addr.clone(),
            &QueryMsg::QueryUserByAddr(QueryUserByAddrMsg {
                user_addr: user_1_addr.to_string(),
            }),
        )
        .unwrap();
    assert_eq!(
        query_user_1_res,
        UserResponse {
            user: User {
                id: Uint64::one(),
                addr: user_1_addr.clone(),
                social_media_handle: None,
                membership_issued_by_me: None,
                fee_config: None,
                fee_share_config: None,
                user_member_count: Uint128::zero()
            }
        }
    );
}
