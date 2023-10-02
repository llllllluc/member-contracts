use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Uint64};

use crate::{
    config::{Config, FeeShareConfig},
    key_holder::KeyHolder,
    thread::Thread,
    thread_msg::ThreadMsg,
    user::User,
    user_holding::UserHolding,
};

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    // Default to sender
    pub admin_addr: Option<String>,
    // Default to sender
    pub registration_admin_addr: Option<String>,
    // Default to sender
    pub protocol_fee_collector_addr: Option<String>,
    // Default to uluna
    // TODO: P1: use noble USDC
    pub fee_denom: Option<String>,
    // Default to 100
    pub max_thread_title_length: Option<Uint64>,
    // Default to 500
    pub max_thread_description_length: Option<Uint64>,
    // Max length of a single thread label
    pub max_thread_label_length: Option<Uint64>,
    // Max number of thread labels
    pub max_number_of_thread_labels: Option<Uint64>,
    // Default to 500
    pub max_thread_msg_length: Option<Uint64>,

    // Protocol fee percentage for key trading
    pub protocol_fee_key_trading_fee_percentage: Option<Uint64>,
    // Protocol fee for starting a new thread
    pub protocol_fee_start_new_thread_fixed_cost: Option<Uint128>,
    // Protocol fee percentage for asking in a thread
    pub protocol_fee_ask_in_thread_fee_percentage: Option<Uint64>,
    // Protocol fee percentage for replying in a thread
    pub protocol_fee_reply_in_thread_fee_percentage: Option<Uint64>,

    // Default key trading fee in my 1 key price percentage
    pub default_trading_fee_percentage_of_key: Option<Uint64>,
    // Default ask me fee in my 1 key price percentage
    pub default_ask_fee_percentage_of_key: Option<Uint64>,
    // Default reply to me in my thread or my msg fee in my 1 key price percentage
    pub default_reply_fee_percentage_of_key: Option<Uint64>,

    // Default key trading fee to key issuer fee percentage
    pub default_key_trading_fee_key_issuer_fee_percentage: Option<Uint64>,
    // Default key trading fee to key holder fee percentage
    pub default_key_trading_fee_key_holder_fee_percentage: Option<Uint64>,

    // Default thread fee to key issuer fee percentage
    pub default_thread_fee_key_issuer_fee_percentage: Option<Uint64>,
    // Default thread fee to key holder fee percentage
    pub default_thread_fee_key_holder_fee_percentage: Option<Uint64>,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig(UpdateConfigMsg),

    // Anyone can register an account
    // But without registering a key they can only buy and sell other people's keys but not issue their own keys
    Register(),

    // Only register admin can link social media for user
    LinkSocialMedia(LinkSocialMediaMsg),

    // Only register admin can register key for user
    // User must link social media first to be eligible for key registration to prevent impersonation
    // This will initialize the user's key and set the supply to 1 owned by the user
    // After that anyone can buy / sell user's key
    RegisterKey(RegisterKeyMsg),

    // Only key issuer can update its key trading fee percentage
    UpdateTradingFeePercentageOfKey(UpdateTradingFeePercentageOfKeyMsg),

    // Only key issuer can update its ask fee percentage
    UpdateAskFeePercentageOfKey(UpdateAskFeePercentageOfKeyMsg),

    // Only key issuer can update its reply fee percentage
    UpdateReplyFeePercentageOfKey(UpdateReplyFeePercentageOfKeyMsg),

    // Only key issuer can update its key trading fee config
    UpdateKeyTradingFeeShareConfig(UpdateKeyTradingFeeShareConfigMsg),

    // Only key issuer can update its thread fee config
    UpdateThreadFeeShareConfig(UpdateThreadFeeShareConfigMsg),

    // Anyone can buy key
    BuyKey(BuyKeyMsg),

    // Anyone can sell key if they have it
    SellKey(SellKeyMsg),

    // TODO: P1: move thread logic to its own contract
    // Anyone can start a new thread
    StartNewThread(StartNewThreadMsg),

    // Key holder can ask question to key issuer in an existing thread or a new thread
    AskInThread(AskInThreadMsg),

    // Key issuer can answer question to key holder in an existing thread
    AnswerInThread(AnswerInThreadMsg),

    // You can reply as long as you hold the key of the thread creator
    // And the key of the msg creator (if replying to a msg)
    ReplyInThread(ReplyInThreadMsg),
    // TODO: P1: new msg to support withdraw question after key issuer not answer for a while, this will send fee back to user
    // TODO: P2: new msg to support open question, anyone can answer, need more thinking
}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub admin_addr: Option<String>,
    pub registration_admin_addr: Option<String>,
    pub protocol_fee_collector_addr: Option<String>,
    pub fee_denom: Option<String>,
    pub max_thread_title_length: Option<Uint64>,
    pub max_thread_description_length: Option<Uint64>,
    pub max_thread_label_length: Option<Uint64>,
    pub max_number_of_thread_labels: Option<Uint64>,
    pub max_thread_msg_length: Option<Uint64>,

    pub protocol_fee_key_trading_fee_percentage: Option<Uint64>,
    pub protocol_fee_start_new_thread_fixed_cost: Option<Uint128>,
    pub protocol_fee_ask_in_thread_fee_percentage: Option<Uint64>,
    pub protocol_fee_reply_in_thread_fee_percentage: Option<Uint64>,

    pub default_trading_fee_percentage_of_key: Option<Uint64>,
    pub default_ask_fee_percentage_of_key: Option<Uint64>,
    pub default_reply_fee_percentage_of_key: Option<Uint64>,

    pub default_key_trading_fee_key_issuer_fee_percentage: Option<Uint64>,
    pub default_key_trading_fee_key_holder_fee_percentage: Option<Uint64>,

    pub default_thread_fee_key_issuer_fee_percentage: Option<Uint64>,
    pub default_thread_fee_key_holder_fee_percentage: Option<Uint64>,
}

#[cw_serde]
pub struct LinkSocialMediaMsg {
    pub user_addr: String,
    pub social_media_handle: String,
}

#[cw_serde]
pub struct RegisterKeyMsg {
    pub user_addr: String,
}

#[cw_serde]
pub struct UpdateTradingFeePercentageOfKeyMsg {
    pub key_issuer_addr: String,
    pub trading_fee_percentage_of_key: Uint64,
}

#[cw_serde]
pub struct UpdateAskFeePercentageOfKeyMsg {
    pub key_issuer_addr: String,
    pub ask_fee_percentage_of_key: Uint64,
}

#[cw_serde]
pub struct UpdateReplyFeePercentageOfKeyMsg {
    pub key_issuer_addr: String,
    pub reply_fee_percentage_of_key: Uint64,
}

#[cw_serde]
pub struct UpdateKeyTradingFeeShareConfigMsg {
    pub key_issuer_addr: String,
    pub key_trading_fee_share_config: FeeShareConfig,
}

#[cw_serde]
pub struct UpdateThreadFeeShareConfigMsg {
    pub key_issuer_addr: String,
    pub thread_fee_share_config: FeeShareConfig,
}

#[cw_serde]
pub struct BuyKeyMsg {
    pub key_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SellKeyMsg {
    pub key_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct StartNewThreadMsg {
    // Thread title
    pub title: String,
    // Thread description
    pub description: String,
    // List of labels
    pub labels: Vec<String>,
}

#[cw_serde]
pub struct AskInThreadMsg {
    // New to start a new thread, default to false
    pub start_new_thread: Option<bool>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_title: Option<String>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_description: Option<String>,
    // If start_new_thread is true, this field must be filled
    // Else start_new_thread is false, this field will be ignored
    pub thread_labels: Option<Vec<String>>,
    // Thread ID to ask question in
    // If start_new_thread is false, this field must be filled
    // Else start_new_thread is true, this field will be ignored
    pub thread_id: Option<Uint64>,
    // The address of the key issuer that the user wants to ask question to
    pub ask_to_addr: String,
    // Question content
    pub content: String,
}

#[cw_serde]
pub struct AnswerInThreadMsg {
    // Thread ID to answer question in
    pub thread_id: Uint64,
    // Answer must be replying to a specific question in a thread
    pub question_id: Uint64,
    // Answer content
    pub content: String,
}

#[cw_serde]
pub struct ReplyInThreadMsg {
    // Thread ID to reply in
    pub thread_id: Uint64,
    // Reply can reply to a specific msg in a thread or the thread itself
    pub reply_to_thread_msg_id: Option<Uint64>,
    // Reply content
    pub content: String,
}

// ========== query ==========

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig(QueryConfigMsg),

    #[returns(UserResponse)]
    QueryUser(QueryUserMsg),

    #[returns(KeySupplyResponse)]
    QueryKeySupply(QueryKeySupplyMsg),

    // Returns all users holding the key, with pagination
    #[returns(KeyHoldersResponse)]
    QueryKeyHolders(QueryKeyHoldersMsg),

    // Returns all keys user currently holds, with pagination
    #[returns(UserHoldingsResponse)]
    QueryUserHoldings(QueryUserHoldingsMsg),

    // QueryCostToBuyKey calculates the price and fee
    #[returns(CostToBuyKeyResponse)]
    QueryCostToBuyKey(QueryCostToBuyKeyMsg),

    // QueryCostToSellKey calculates the price and fee
    #[returns(CostToSellKeyResponse)]
    QueryCostToSellKey(QueryCostToSellKeyMsg),

    // QueryCostToStartNewThread calculates the fee needed to ask a question
    #[returns(CostToStartNewThreadResponse)]
    QueryCostToStartNewThread(QueryCostToStartNewThreadMsg),

    // QueryCostToAsk calculates the fee needed to ask a question
    #[returns(CostToAskResponse)]
    QueryCostToAsk(QueryCostToAskMsg),

    // NOTE: answer has no cost

    // QueryCostToAsk calculates the fee needed to reply to a thread or a msg in a thread
    #[returns(CostToReplyResponse)]
    QueryCostToReply(QueryCostToReplyMsg),

    #[returns(IDsOfAllThreadsUserBelongToResponse)]
    QueryIDsOfAllThreadsUserBelongTo(QueryIDsOfAllThreadsUserBelongToMsg),

    #[returns(IDsOfAllThreadsUserCreatedResponse)]
    QueryIDsOfAllThreadsUserCreated(QueryIDsOfAllThreadsUserCreatedMsg),

    #[returns(ThreadsByIDsResponse)]
    QueryThreadsByIDs(QueryThreadsByIDsMsg),

    #[returns(ThreadMsgsByIDsResponse)]
    QueryThreadMsgsByIDs(QueryThreadMsgsByIDsMsg),
}

#[cw_serde]
pub struct QueryConfigMsg {}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct QueryUserMsg {
    pub user_addr: String,
}

#[cw_serde]
pub struct UserResponse {
    pub user: User,
}

#[cw_serde]
pub struct QueryKeySupplyMsg {
    pub key_issuer_addr: String,
}

#[cw_serde]
pub struct KeySupplyResponse {
    pub supply: Uint128,
}

#[cw_serde]
pub struct QueryKeyHoldersMsg {
    pub key_issuer_addr: String,
    pub start_after_user_addr: Option<String>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct KeyHoldersResponse {
    pub key_holders: Vec<KeyHolder>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryUserHoldingsMsg {
    pub user_addr: String,
    pub start_after_key_issuer_addr: Option<String>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct UserHoldingsResponse {
    pub user_holdings: Vec<UserHolding>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryCostToBuyKeyMsg {
    pub key_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct CostToBuyKeyResponse {
    // Price is total price for buy amount of key, not the price per key
    pub price: Uint128,
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Price + protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryCostToSellKeyMsg {
    pub key_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct CostToSellKeyResponse {
    // Price is total price for sell amount of key, not the price per key
    pub price: Uint128,
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryCostToStartNewThreadMsg {
    pub description_len: Uint64,
}

#[cw_serde]
pub struct CostToStartNewThreadResponse {
    pub protocol_fee: Uint128,
}

#[cw_serde]
pub struct QueryCostToAskMsg {
    // The address of the key issuer that the user wants to ask question to
    pub ask_to_addr: String,
    // Number of characters in question content
    pub content_len: Uint64,
}

#[cw_serde]
pub struct CostToAskResponse {
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryCostToReplyMsg {
    // The address of the key issuer that the user wants to reply to
    // Either a msg (reply or question or answer) owner or a thread owner
    pub reply_to_addr: String,
    // Number of characters in question content
    pub content_len: Uint64,
}

#[cw_serde]
pub struct CostToReplyResponse {
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to key issuer
    pub key_issuer_fee: Uint128,
    // Fee paid to all key holders
    pub key_holder_fee: Uint128,
    // Protocol fee + key issuer fee + key holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryIDsOfAllThreadsUserBelongToMsg {
    pub user_addr: String,
    pub start_after_thread_id: Option<Uint64>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct IDsOfAllThreadsUserBelongToResponse {
    pub thread_ids: Vec<Uint64>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryIDsOfAllThreadsUserCreatedMsg {
    pub user_addr: String,
    pub start_after_thread_id: Option<Uint64>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct IDsOfAllThreadsUserCreatedResponse {
    pub thread_ids: Vec<Uint64>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryThreadsByIDsMsg {
    pub thread_ids: Vec<Uint64>,
}

#[cw_serde]
pub struct ThreadsByIDsResponse {
    pub threads: Vec<Thread>,
}

#[cw_serde]
pub struct QueryThreadMsgsByIDsMsg {
    pub thread_and_thread_msg_ids: Vec<(Uint64, Uint64)>,
}

#[cw_serde]
pub struct ThreadMsgsByIDsResponse {
    pub thread_msgs: Vec<ThreadMsg>,
}
