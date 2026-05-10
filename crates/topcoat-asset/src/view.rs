use std::collections::HashMap;

use topcoat_core::context::{Cx, app_state};
use topcoat_view::runtime::{Formatter, Fragment};

use crate::Asset;

pub struct AssetFragmentResolver {
    lookup: HashMap<Asset, String>,
}

impl AssetFragmentResolver {
    pub fn new(lookup: HashMap<Asset, String>) -> Self {
        Self { lookup }
    }

    pub fn resolve(&self, asset: &Asset) -> &str {
        match self.lookup.get(asset) {
            Some(result) => result,
            _ => panic!("failed to resolve asset {asset:?}"),
        }
    }
}

impl Fragment for Asset {
    fn fmt(&self, cx: &Cx, f: &mut Formatter<'_>) {
        f.write_str(app_state::<AssetFragmentResolver>(cx).resolve(self))
    }

    fn fmt_unescaped(&self, cx: &Cx, f: &mut Formatter<'_>) {
        f.write_str_unescaped(app_state::<AssetFragmentResolver>(cx).resolve(self))
    }
}
