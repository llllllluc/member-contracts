use cosmwasm_std::Uint128;

use thread::msg::{CostToBuyKeyResponse, QueryCostToBuyKeyMsg, QueryMsg};

pub mod helpers;
use crate::helpers::{
    assert_key_supply, link_social_media_and_register_key, print_balance, proper_instantiate,
    register_user, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_query_cost_to_buy_key() {
    let (
        mut app,
        cw_thread_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let uint_128_amount_30 = Uint128::from(30_u8);

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_register_key(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        &user_1_addr,
        SOCIAL_MEDIA_HANDLE_1,
    );

    assert_key_supply(&app, &cw_thread_contract_addr, &user_1_addr, Uint128::one());

    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 buy 30 amount of its own keys
    let query_user_1_simulate_buy_key_res: CostToBuyKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();

    assert_eq!(
        query_user_1_simulate_buy_key_res,
        CostToBuyKeyResponse {
            price: Uint128::from(590_937_u32),
            protocol_fee: Uint128::from(2954_u32),
            key_issuer_fee: Uint128::from(14_773_u32),
            key_holder_fee: Uint128::from(14_773_u32),
            total_needed_from_user: Uint128::from(623_437_u32),
        }
    );
}