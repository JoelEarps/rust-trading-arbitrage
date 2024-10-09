use std::collections::HashMap;
// Data will be consistent here, always of length two and therefore we can do something a bit more hard coded
pub(crate) fn remove_duplicate_tickers(raw_rates: &mut HashMap<String, String>) {
    println!("Removing duplicate tickers to ensure clean node map");
    raw_rates.retain(|key, _value| {
        let mut split_key: Vec<&str> = key.split("-").collect();
        split_key.dedup();
        println!("{:?}", &split_key.len());
        match split_key.len() {
            1 => { println!("Conversions between the same tickers found");
                   false
                }
            2 => {
                println!("Valid pairing found");
                true
            }
            _ => {
                println!("Invalid input");
                false
            }
        }
    });
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_duplicates_with_replica_data() {
        // Test data the same as what would be pulled from the API
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BORG-BTC".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("BTC-BTC".to_string(), "1.00000000".to_string());
        exchange_rates.insert("DAI-BTC".to_string(), "61194.73626107".to_string());
        exchange_rates.insert("EUR-BORG".to_string(), "0.14846482".to_string());
        exchange_rates.insert("BORG-DAI".to_string(), "6.10395983".to_string());
        exchange_rates.insert("DAI-EUR".to_string(), "1.10578631".to_string());
        exchange_rates.insert("EUR-DAI".to_string(), "0.89846283".to_string());
        exchange_rates.insert("BTC-BORG".to_string(), "0.00000269".to_string());
        exchange_rates.insert("EUR-BTC".to_string(), "57157.97132034".to_string());
        exchange_rates.insert("BTC-EUR".to_string(), "0.00001739".to_string());
        exchange_rates.insert("BTC-DAI".to_string(), "0.00001586".to_string());
        exchange_rates.insert("BORG-BORG".to_string(), "1.00000000".to_string());
        exchange_rates.insert("EUR-EUR".to_string(), "1.00000000".to_string());
        exchange_rates.insert("BORG-EUR".to_string(), "6.53505020".to_string());
        exchange_rates.insert("DAI-BORG".to_string(), "0.16276449".to_string());
        exchange_rates.insert("DAI-DAI".to_string(), "1.00000000".to_string());
        assert_eq!(exchange_rates.len(), 16);
        remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(exchange_rates.len(), 12);
    }
    #[test]
    // Random Test data - all the same should be zero
    fn test_duplicates_with_no_replica_data() {
        // Test data the same as what would be pulled from the API
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BORG-BTC".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("DAI-BTC".to_string(), "61194.73626107".to_string());
        exchange_rates.insert("EUR-BORG".to_string(), "0.14846482".to_string());
        exchange_rates.insert("BORG-DAI".to_string(), "6.10395983".to_string());
        exchange_rates.insert("DAI-EUR".to_string(), "1.10578631".to_string());
        exchange_rates.insert("EUR-DAI".to_string(), "0.89846283".to_string());
        exchange_rates.insert("BTC-BORG".to_string(), "0.00000269".to_string());
        exchange_rates.insert("EUR-BTC".to_string(), "57157.97132034".to_string());
        exchange_rates.insert("BTC-EUR".to_string(), "0.00001739".to_string());
        exchange_rates.insert("BTC-DAI".to_string(), "0.00001586".to_string());
        exchange_rates.insert("BORG-EUR".to_string(), "6.53505020".to_string());
        exchange_rates.insert("DAI-BORG".to_string(), "0.16276449".to_string());
        assert_eq!(exchange_rates.len(), 12);
        remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(exchange_rates.len(), 12);
    }
    #[test]
    // Random data, no duplicates
    fn test_duplicates_with_all_replica_tickers() {
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("BTC-BTC".to_string(), "1.00000000".to_string());
        exchange_rates.insert("BORG-BORG".to_string(), "1.00000000".to_string());
        exchange_rates.insert("EUR-EUR".to_string(), "1.00000000".to_string());
        exchange_rates.insert("DAI-DAI".to_string(), "1.00000000".to_string());
        assert_eq!(exchange_rates.len(), 4);
        remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(exchange_rates.len(), 0);
    }
    #[test]
    // Random data none ticker related - no splitting custom error?
    fn test_duplicates_with_random_data() {
        let mut exchange_rates: HashMap<String, String> = HashMap::new();
        exchange_rates.insert("Test1".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("Test2".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("Test3".to_string(), "370331.49347896".to_string());
        exchange_rates.insert("Test4".to_string(), "370331.49347896".to_string());
        assert_eq!(exchange_rates.len(), 4);
        remove_duplicate_tickers(&mut exchange_rates);
        assert_eq!(exchange_rates.len(), 0);
    }
}
