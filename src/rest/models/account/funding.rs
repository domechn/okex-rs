use super::super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FundingBalanceRequest {
    pub ccy: Option<String>,
}

impl FundingBalanceRequest {
    pub fn multiple<S>(currencies: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            ccy: Some(
                currencies
                    .into_iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FundingBalanceResponse {
    pub bal: String,
    pub ccy: String,
}

impl Request for FundingBalanceRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/asset/balances";
    const HAS_PAYLOAD: bool = false;
    type Response = [FundingBalanceResponse; 1];
}
