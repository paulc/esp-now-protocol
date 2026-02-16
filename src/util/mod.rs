mod format_mac;
mod js;
mod view;

pub use format_mac::{format_mac, parse_mac};
pub use view::display_vec;

#[cfg(feature = "js")]
pub use js::{buf_to_array, js_format_mac, js_parse_mac};
