use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::Path,
};

use sha2::{Digest, Sha256};

use crate::{MANIFEST_NAME, MANIFEST_VERSION, Manifest, ManifestEntry, RawAsset};

pub struct Bundler {}

impl Bundler {
    /// Scan `binary` for embedded assets and sync them into `out_dir`.
    ///
    /// If `out_dir` already contains a `manifest.toml`, it is loaded and used
    /// to skip copying files whose content hash hasn't changed. Files that
    /// were present in the old manifest but are no longer referenced by the
    /// new one are removed.
    pub fn bundle(binary: &[u8], out_dir: impl AsRef<Path>) -> io::Result<()> {
        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir)?;

        let manifest_path = out_dir.join(MANIFEST_NAME);
        let existing: HashMap<_, _> = match Manifest::load(&manifest_path) {
            Ok(manifest) => manifest
                .assets
                .into_iter()
                .map(|entry| (entry.id, entry))
                .collect(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => HashMap::new(),
            Err(e) => return Err(e),
        };

        let assets = RawAsset::find_in_binary(binary);
        let mut entries = Vec::with_capacity(assets.len());
        let mut kept_files = HashSet::with_capacity(assets.len());

        for asset in assets {
            let src = asset.resolved_path();
            let bytes = fs::read(&src)?;
            let digest = Sha256::digest(&bytes);
            let hash = digest
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            let short_hash = &hash[..8];

            let stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or("asset");
            let file = match src.extension().and_then(|e| e.to_str()) {
                Some(ext) => format!("{stem}.{short_hash}.{ext}"),
                None => format!("{stem}.{short_hash}"),
            };

            let id = asset.id();
            let dst = out_dir.join(&file);
            let unchanged = existing
                .get(&id)
                .is_some_and(|prev| prev.hash == hash && prev.file == file);

            if !unchanged || !dst.exists() {
                fs::copy(&src, &dst)?;
            }

            kept_files.insert(file.clone());
            entries.push(ManifestEntry { id, file, hash });
        }

        for entry in existing.values() {
            if !kept_files.contains(&entry.file) {
                let path = out_dir.join(&entry.file);
                match fs::remove_file(&path) {
                    Ok(()) => {}
                    Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e),
                }
            }
        }

        let manifest = Manifest {
            version: MANIFEST_VERSION,
            assets: entries,
        };
        manifest.save(manifest_path)?;

        Ok(())
    }
}
