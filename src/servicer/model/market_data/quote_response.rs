mod equity;
mod forex;
mod future;
mod future_option;
mod index;
mod mutual_fund;
mod option;
mod quote_error;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, TimestampMilliSeconds};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum QuoteResponse {
    Equity(equity::EquityResponse),
    Forex(forex::ForexResponse),
    Future(future::FutureResponse),
    FutureOption(future_option::FutureOptionResponse),
    Index(index::IndexResponse),
    MutualFund(mutual_fund::MutualFundResponse),
    Option(option::OptionResponse),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/MarketData/QuoteResponse.json"
        ));

        let val = serde_json::from_str::<HashMap<String, QuoteResponse>>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
