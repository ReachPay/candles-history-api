use serde::{Deserialize, Serialize};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel{
    #[serde(rename = "CandlesGrpc")]
    pub candles_grpc: String
}