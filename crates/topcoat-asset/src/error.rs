use std::{io, path::PathBuf};

use crate::RawAsset;

pub type Result = core::result::Result<(), AssetError>;

#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("io error for asset at {}: {source}", asset.resolved_path().display())]
    AssetIo {
        asset: RawAsset,
        #[source]
        source: io::Error,
    },
    #[error("io error for manifest at {}: {source}", path.display())]
    ManifestIo {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
