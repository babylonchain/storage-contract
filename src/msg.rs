use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SaveData { data: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CheckDataResponse)]
    CheckData {
        // hex encoded hash of given data
        data_hash: String,
    },
}

#[cw_serde]
pub struct CheckDataResponse {
    pub height: u64,
    pub timestamp: u64,
    pub finalized: bool,
    pub save_epoch: u64,
    pub latest_finalized_epoch: u64,
}
