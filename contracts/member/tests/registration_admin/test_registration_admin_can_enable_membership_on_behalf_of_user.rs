use cosmwasm_std::{Uint128, Uint64};

use member_pkg::{
    msg::{QueryMsg, QueryUserByAddrMsg, UserResponse},
    user::{Member, Membership, MembershipIssuedByMe, User},
};

use crate::helpers::{
    assert_member_count, assert_members, assert_membership_supply, assert_memberships,
    enable_membership, link_social_media, proper_instantiate, register_user, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_registration_admin_can_enable_membership_on_behalf_of_user() {
    let (mut app, cw_member_contract_addr, _, registration_admin_addr, _, user_1_addr, _) =
        proper_instantiate();
    register_user(&mut app, &cw_member_contract_addr, &user_1_addr).unwrap();
    let user_1_id = Uint64::one();

    link_social_media(
        &mut app,
        &cw_member_contract_addr,
        &registration_admin_addr,
        user_1_id,
        SOCIAL_MEDIA_HANDLE_1,
    )
    .unwrap();
    enable_membership(
        &mut app,
        &cw_member_contract_addr,
        &registration_admin_addr,
        user_1_id,
    )
    .unwrap();

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
                id: user_1_id,
                addr: user_1_addr.clone(),
                social_media_handle: Some(SOCIAL_MEDIA_HANDLE_1.to_string()),
                membership_issued_by_me: Some(MembershipIssuedByMe {
                    membership_supply: Uint128::one(),
                    member_count: Uint128::one()
                }),
                fee_config: None,
                fee_share_config: None,
                user_member_count: Uint128::one()
            }
        }
    );

    assert_membership_supply(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    assert_member_count(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    assert_memberships(
        &app,
        &cw_member_contract_addr,
        Uint64::one(),
        vec![Membership {
            issuer_user_id: Uint64::one(),
            amount: Uint128::one(),
        }],
        1,
        1,
    );

    assert_members(
        &app,
        &cw_member_contract_addr,
        user_1_id,
        vec![Member {
            member_user_id: user_1_id,
            amount: Uint128::one(),
        }],
        1,
        1,
    );
}
