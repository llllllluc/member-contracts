use cosmwasm_std::{Coin, Uint128, Uint64};
use cw_multi_test::Executor;

use member::ContractError;
use member_pkg::msg::{
    BuyMembershipMsg, CostToBuyMembershipResponse, ExecuteMsg, QueryCostToBuyMembershipMsg,
    QueryCostToSellMembershipMsg, QueryMsg, SellMembershipMsg,
};

use crate::helpers::{
    assert_err, enable_membership, get_fund_from_faucet, link_social_media, print_balance,
    proper_instantiate, register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_sell_membership_insufficient_funds() {
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
    let uint_128_amount_10 = Uint128::from(10_u8);

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
    println!(
        "query_user_1_simulate_buy_membership_res {:?}",
        query_user_1_simulate_buy_membership_res
    );

    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_buy_membership_res.total_needed_from_user,
    );

    app.execute_contract(
        user_1_addr.clone(),
        cw_member_contract_addr.clone(),
        &ExecuteMsg::BuyMembership(BuyMembershipMsg {
            membership_issuer_user_id: user_1_id,
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_buy_membership_res.total_needed_from_user,
        }],
    )
    .unwrap();

    print_balance(
        &app,
        &cw_member_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 tries to sell 10 amount of its own memberships but fails because it didn't pay enough protocol fee
    let query_user_1_simulate_sell_membership_res: CostToBuyMembershipResponse = app
        .wrap()
        .query_wasm_smart(
            cw_member_contract_addr.clone(),
            &QueryMsg::QueryCostToSellMembership(QueryCostToSellMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_10,
            }),
        )
        .unwrap();

    get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());

    assert_err(
        app.execute_contract(
            user_1_addr.clone(),
            cw_member_contract_addr.clone(),
            &ExecuteMsg::SellMembership(SellMembershipMsg {
                membership_issuer_user_id: user_1_id,
                amount: uint_128_amount_10,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: Uint128::one(),
            }],
        ),
        ContractError::InsufficientFundsToPayDuringSell {
            needed: query_user_1_simulate_sell_membership_res.total_needed_from_user,
            available: Uint128::one(),
        },
    );
}
