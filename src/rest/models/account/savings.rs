use super::super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SavingsBalanceRequest {
    pub ccy: Option<String>,
}

impl SavingsBalanceRequest {
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
pub struct SavingsBalanceResponse {
    pub ccy: String,
    pub amt: String,
}

impl Request for SavingsBalanceRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/api/v5/finance/savings/balance";
    const HAS_PAYLOAD: bool = false;
    type Response = Vec<SavingsBalanceResponse>;
}
