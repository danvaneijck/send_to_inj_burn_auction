use cw20::Cw20ReceiveMsg;
use serde::{Deserialize, Serialize};
use crate::state::{Asset};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub admin: String,
    pub adapter_contract: String,
    pub burn_auction_subaccount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    SendNative {asset: Asset},
    UpdateAdmin { admin: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetConfig {},
}