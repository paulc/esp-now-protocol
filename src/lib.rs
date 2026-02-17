#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod monitor;
pub mod types;
pub mod util;

pub use types::*;
pub use util::{format_mac, parse_mac};

#[cfg(feature = "js")]
pub use util::register_espnow;

#[cfg(test)]
mod tests;

pub const VERSION: u32 = 0;
pub const MAX_DATA_LEN: usize = 250;
