//! Augur venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Augur";

/// Venue category.
pub const VENUE_TYPE: &str = "Ethereum";

/// Strategies mapped to this venue on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Resolution Sniper",
    "Orderbook Imbalance",
];
