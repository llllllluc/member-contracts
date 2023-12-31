use cosmwasm_std::Uint64;
use cw_multi_test::Executor;

use member::ContractError;
use member_pkg::msg::{EnableMembershipMsg, ExecuteMsg, LinkSocialMediaMsg};

use crate::helpers::{assert_err, proper_instantiate, register_user, SOCIAL_MEDIA_HANDLE_1};

#[test]
fn test_user_cannot_enable_membership_by_itself() {
    let (mut app, cw_member_contract_addr, _, _, _, user_1_addr, _) = proper_instantiate();
    register_user(&mut app, &cw_member_contract_addr, &user_1_addr).unwrap();
    assert_err(
        app.execute_contract(
            user_1_addr.clone(),
            cw_member_contract_addr.clone(),
            &ExecuteMsg::LinkSocialMedia(LinkSocialMediaMsg {
                user_id: Uint64::one(),
                social_media_handle: SOCIAL_MEDIA_HANDLE_1.to_string(),
            }),
            &[],
        ),
        ContractError::OnlyRegistrationAdminCanLinkSocialMediaOnBehalfOfUser {},
    );
    assert_err(
        app.execute_contract(
            user_1_addr.clone(),
            cw_member_contract_addr.clone(),
            &ExecuteMsg::EnableMembership(EnableMembershipMsg {
                user_id: Uint64::one(),
            }),
            &[],
        ),
        ContractError::OnlyRegistrationAdminCanEnableMembershipOnBehalfOfUser {},
    );
}
