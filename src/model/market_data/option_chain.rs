use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use super::quote_response::option::ExpirationType;
use super::quote_response::option::SettlementType;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Underlying,
    pub strategy: Strategy,
    pub interval: i64,
    pub is_delayed: bool,
    pub is_index: bool,
    pub days_to_expiration: i64,
    pub interest_rate: i64,
    pub underlying_price: f64,
    pub volatility: i64,
    pub call_exp_date_map: HashMap<String, HashMap<String, OptionContract>>,
    pub put_exp_date_map: HashMap<String, HashMap<String, OptionContract>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    pub ask: i64,
    pub ask_size: i64,
    pub bid: i64,
    pub bid_size: i64,
    pub change: i64,
    pub close: i64,
    pub delayed: bool,
    pub description: String,
    pub exchange_name: ExchangeName,
    pub fifty_two_week_high: i64,
    pub fifty_two_week_low: i64,
    pub high_price: f64,
    pub last: i64,
    pub low_price: f64,
    pub mark: i64,
    pub mark_change: i64,
    pub mark_percent_change: i64,
    pub open_price: f64,
    pub percent_change: i64,
    pub quote_time: i64,
    pub symbol: String,
    pub total_volume: i64,
    pub trade_time: i64,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    pub put_call: PutCall,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub last_price: f64,
    pub mark_price: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
    pub trade_date: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub net_change: i64,
    pub volatility: i64,
    pub delta: i64,
    pub gamma: i64,
    pub theta: i64,
    pub vega: i64,
    pub rho: i64,
    pub time_value: i64,
    pub open_interest: i64,
    pub is_in_the_money: bool,
    pub theoretical_option_value: i64,
    pub theoretical_volatility: i64,
    pub is_mini: bool,
    pub is_non_standard: bool,
    pub option_deliverables_list: Vec<OptionDeliverable>,
    pub strike_price: f64,
    pub expiration_date: String,
    pub days_to_expiration: i64,
    pub expiration_type: ExpirationType,
    pub last_trading_day: i64,
    pub multiplier: i64,
    pub settlement_type: SettlementType,
    pub deliverable_note: String,
    pub is_index_option: bool,
    pub percent_change: i64,
    pub mark_change: i64,
    pub mark_percent_change: i64,
    pub is_penny_pilot: bool,
    pub intrinsic_value: i64,
    pub option_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverable {
    pub symbol: String,
    pub asset_type: String,
    pub deliverable_units: String,
    pub currency_type: String,
}

/// Available values : `SINGLE`, `ANALYTICAL`, `COVERED`, `VERTICAL`, `CALENDAR`, `STRANGLE`, `STRADDLE`, `BUTTERFLY`, `CONDOR`, `DIAGONAL`, `COLLAR`, `ROLL`
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Strategy {
    #[default]
    Single,
    Analytical,
    Covered,
    Vertical,
    Calendar,
    Strangle,
    Straddle,
    Butterfly,
    Condor,
    Diagonal,
    Collar,
    Roll,
}

/// Available values : IND, ASE, NYS, NAS, NAP, PAC, OPR, BATS
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ExchangeName {
    #[default]
    Ind,
    Ase,
    Nys,
    Nas,
    Nap,
    Pac,
    Opr,
    Bats,
}

/// Available values : `PUT`, `CALL`
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PutCall {
    #[default]
    Put,
    Call,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/OptionChain.json"
        ));

        let val = serde_json::from_str::<OptionChain>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
