use crate::ContractError;

pub fn decode_hex(data: &str) -> Result<Vec<u8>, ContractError> {
    hex::decode(data).map_err(|_| ContractError::HexDecodingError {})
}
