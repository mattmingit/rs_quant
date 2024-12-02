//! # Yahoo! Finance Connection
//! This module provides a utility to establish a connection to Yahoo! Finance using the
//! `yahoofinance` crate (the `yahoofinance` crate is a fork of `yahoo_finance_api`, which is maintained by the original developers. The fork just extends its functionality). It simplifies the initialization of a Yahoo! Finance connector and makes it accessible through a dedicated struct.
//! ## Structs
//!
//! ### `YahooFinance`
//! Represents a Yahoo! Finance connection and contains the following field:
//! - `connector`: An instance of `YahooConnector` used to interact with Yahoo! Finance's API.
//!
//! ## Implementation
//!
//! ### `YahooFinance::connector`
//! Creates and initializes a new instance of `YahooFinance` with a valid `YahooConnector`.
//!
//! #### Returns
//! - A new instance of `YahooFinance` if the connection is successfully established.
//!
//! #### Panics
//! - Panics if the `YahooConnector::new()` method fails to create a connection. This indicates
//!   an issue with the Yahoo! Finance API or the crate configuration.
//!
//! #### Example
//! ```rust
//! use rs_quant::data::provider::YahooFinance;
//! let connection = YahooFinance::connector();
//! ```
//! ## Notes
//! - The `YahooConnector` is a part of the `yahoofinance` crate (a fork of the `yahoo_finance_api`), which must be addes as a dependency in your `Cargo.toml`.
//! - Ensure you handle potential failures during initialization by configuring your application appropriately.

use yahoofinance::YahooConnector;

/// Define the connection using yahoo_finance_api crate
pub struct YahooFinance {
    pub connector: YahooConnector,
}

impl YahooFinance {
    pub fn connector() -> Self {
        let c = YahooConnector::new().expect("Failed to build connection with yahoo! finance.");
        Self { connector: c }
    }
}
