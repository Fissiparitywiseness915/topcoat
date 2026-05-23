pub mod ast;
mod bind_attribute;
mod expr;
mod signal;

pub use bind_attribute::*;
pub use expr::*;
pub use signal::*;

use topcoat_asset::{Asset, asset};

pub const SCRIPT: Asset = asset!("browser/dist/index.mjs", rename: "topcoat");
