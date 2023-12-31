use cosmwasm_std::{Uint128, Uint64};

use member_pkg::msg::{CostToBuyMembershipResponse, QueryCostToBuyMembershipMsg, QueryMsg};

use crate::helpers::{
    assert_member_count, assert_membership_supply, enable_membership, link_social_media,
    print_balance, proper_instantiate, register_user, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_query_cost_to_buy_membership() {
    let (
        mut app,
        cw_member_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let uint_128_amount_30 = Uint128::from(30_u8);

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
    assert_membership_supply(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    assert_member_count(&app, &cw_member_contract_addr, user_1_id, Uint128::one());

    print_balance(
        &app,
        &cw_member_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 buy 30 amount of its own memberships
    let query_user_1_simulate_buy_membership_res: CostToBuyMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_member_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyMembership(QueryCostToBuyMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    assert_eq!(
        query_user_1_simulate_buy_membership_res,
        CostToBuyMembershipResponse {
            price: Uint128::from(590_937_u32),
            protocol_fee: Uint128::from(2954_u32),
            issuer_fee: Uint128::from(23_636_u32),
            all_members_fee: Uint128::from(5_909_u32),
            total_needed_from_user: Uint128::from(623_436_u32),
        }
    );
}
