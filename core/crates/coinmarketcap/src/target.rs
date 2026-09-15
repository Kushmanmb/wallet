use gem_client::{Target, build_path_with_query};
use primitives::Currency;

use crate::model::USD_ID;

#[derive(Clone, Debug)]
pub enum CoinMarketCapTarget {
    LatestListings { limit: usize },
    TrendingListings { limit: usize },
    Info { key: String, value: String },
    FiatMap,
    FiatRates { currencies: Vec<Currency> },
}

impl Target for CoinMarketCapTarget {
    fn path(&self) -> String {
        match self {
            Self::LatestListings { limit } => format!("/v1/cryptocurrency/listings/latest?limit={limit}"),
            Self::TrendingListings { limit } => format!("/v1/cryptocurrency/trending/latest?limit={limit}"),
            Self::Info { key, value } => build_path_with_query("/v2/cryptocurrency/info", &[(key, value)]),
            Self::FiatMap => "/v1/fiat/map".to_string(),
            Self::FiatRates { currencies } => build_path_with_query(
                "/v2/tools/price-conversion",
                &[
                    ("amount", "1".to_string()),
                    ("id", USD_ID.to_string()),
                    ("convert", currencies.iter().map(Currency::as_ref).collect::<Vec<_>>().join(",")),
                ],
            ),
        }
    }
}
