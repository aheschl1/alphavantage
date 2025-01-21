use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as SerdeError;
use crate::deserialize::{from_str, DATE_FORMAT, float_to_string};

/// Represents dividend data for a specific symbol.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct DividendEntry {
    /// Ex-dividend date.
    #[serde(deserialize_with = "parse_optional_date")]
    ex_dividend_date: Option<NaiveDate>,
    /// Declaration date.
    #[serde(deserialize_with = "parse_optional_date")]
    declaration_date: Option<NaiveDate>,
    /// Record date.
    #[serde(deserialize_with = "parse_optional_date")]
    record_date: Option<NaiveDate>,
    /// Payment date.
    #[serde(deserialize_with = "parse_optional_date")]
    pub payment_date: Option<NaiveDate>,
    /// Dividend amount.
    #[serde(deserialize_with = "from_str", serialize_with = "float_to_string")]
    pub amount: f64,
}

/// Represents a set of dividend entries for a symbol.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct DividendResults {
    /// The symbol associated with the dividend data.
    symbol: String,
    /// The list of dividend entries.
    pub data: Vec<DividendEntry>,
}

/// Custom deserializer for optional dates.
fn parse_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;
    if let Some(ref date_str) = date_str {
        if date_str == "None" {
            Ok(None)
        } else {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map(Some)
                .map_err(SerdeError::custom)
        }
    } else {
        Ok(None)
    }
}

pub(crate) mod parser {
    use super::*;
    use crate::error::Error;
    use std::io::Read;

    pub fn parse(reader: impl Read) -> Result<DividendResults, Error> {
        let results: DividendResults = serde_json::from_reader(reader)?;
        Ok(results)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parse_dividend_data() {
        let data: &[u8] = include_bytes!("../tests/json/dividends_ibm.json");

        let results = parser::parse(BufReader::new(data)).expect("failed to parse dividend data");
        assert_eq!(results.symbol, "IBM");
        assert_eq!(results.data.len(), 19);
        assert_eq!(
            results.data[0],
            DividendEntry {
                ex_dividend_date: Some(NaiveDate::from_ymd_opt(2024, 11, 12).unwrap()),
                declaration_date: Some(NaiveDate::from_ymd_opt(2024, 10, 30).unwrap()),
                record_date: Some(NaiveDate::from_ymd_opt(2024, 11, 12).unwrap()),
                payment_date: Some(NaiveDate::from_ymd_opt(2024, 12, 10).unwrap()),
                amount: 1.67,
            }
        );
        assert_eq!(
            results.data[18],
            DividendEntry {
                ex_dividend_date: Some(NaiveDate::from_ymd_opt(2020, 5, 7).unwrap()),
                declaration_date: None,
                record_date: None,
                payment_date: None,
                amount: 1.63,
            }
        );
    }
}
