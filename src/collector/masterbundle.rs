use std::path::{Component, Path, PathBuf};

use anyhow::Context;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// Paths that the MasterBundle parser should ignore.
pub const DISALLOWED_PATHS: [&str; 8] = [
    "Objects", "Effects", "Terrain", "Assets", "Grass", "Trees", "PBSNPCS", "Logs",
];

/// A MasterBundle manifest file containing metadata for a [`MasterBundle`].
///
/// This manifest file contains important fields such as `Assets`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "Assets")]
    pub assets: Vec<String>,
}

#[derive(Debug)]
pub struct MasterBundle {
    /// The name of the [`MasterBundle`], gotten from `MasterBundle.dat`.
    pub name: String,
    // All of the assets inside the [`MasterBundle`] manifest.
    pub assets: Vec<PathBuf>,
}

#[derive(Debug, Default)]
pub struct MasterBundleData {
    pub name: String,
    pub asset_prefix: String,
}

impl MasterBundle {
    /// Creates a new [`MasterBundle`].
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();

        let mut masterbundle_location = PathBuf::from(path);
        masterbundle_location.push("MasterBundle.dat");

        let masterbundle_data = Self::parse_masterbundle_data(&masterbundle_location)
            .context("failed to parse MasterBundle data")?;
        let assets = Self::parse_assets(
            path,
            &masterbundle_data.name,
            &masterbundle_data.asset_prefix,
        )
        .context("failed to parse MasterBundle assets")?;

        Ok(Self {
            name: masterbundle_data.name,
            assets,
        })
    }

    /// Gathers the [`MasterBundle`] assets.
    pub fn parse_assets<P: AsRef<Path>>(
        path: P,
        name: &str,
        asset_prefix: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
        let mut manifest_location = path.as_ref().to_owned();
        manifest_location.push(format!("{}.manifest", name));

        let manifest_file = std::fs::File::open(&manifest_location)
            .with_context(|| format!("Failed to open file: {}", manifest_location.display()))?;
        let manifest: Manifest =
            serde_yml::from_reader(manifest_file).context("Failed to parse manifest file")?;

        let assets: Vec<PathBuf> = manifest.assets.into_iter().map(PathBuf::from).collect();
        let assets: Vec<&Path> = assets
            .iter()
            .filter_map(|asset| asset.strip_prefix(asset_prefix).ok())
            .filter(|asset| {
                let mut components = asset.components();
                !matches!(components.next(), Some(Component::Normal(component)) if DISALLOWED_PATHS.contains(&component.to_str().unwrap()))
            })
            .collect();
        let assets: Vec<PathBuf> = assets.into_iter().map(PathBuf::from).collect();

        tracing::debug!("Got assets: {:#?}", assets);

        Ok(assets)
    }

    /// Reads a `MasterBundle.dat` file and returns its name and asset prefix
    pub fn parse_masterbundle_data(path: &Path) -> anyhow::Result<MasterBundleData> {
        let path: PathBuf = path.into();
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        let mut data = MasterBundleData::default();

        for line in contents.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Asset_Bundle_Name" => data.name = value.into(),
                "Asset_Prefix" => data.asset_prefix = value.into(),
                _ => {}
            }
        }

        Ok(data)
    }

    /// Get all paths from the MasterBundle
    pub fn get_paths(&self) -> Vec<&Path> {
        let paths: Vec<&Path> = self
            .assets
            .iter()
            .filter_map(|asset| asset.parent())
            .filter(|path| !path.as_os_str().is_empty() && path != &Path::new("."))
            .collect();
        let paths: Vec<_> = paths.into_iter().sorted().dedup_by(|a, b| a == b).collect();

        paths
    }
}
