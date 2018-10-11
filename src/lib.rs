#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

/// Import `tachyons.css` framework
pub const TACHYONS: &'static str =
  include_str!("../tachyons-custom/css/tachyons.css");

/// Import the default configuration for `tachyons.css`.
pub const TACHYONS_DEFAULT: &'static str = include_str!("variables.css");
