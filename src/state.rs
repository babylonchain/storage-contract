use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

#[cw_serde]
pub struct StoredData {
    pub data: String,
    pub btc_height: u64,
    pub btc_timestamp: u64,
    pub saved_at_btc_epoch: u64,
}

pub const STORED_DATA: Map<&str, StoredData> = Map::new("stored_data");
