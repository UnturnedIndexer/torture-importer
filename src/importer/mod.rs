pub mod asset;
pub mod workshop;

use std::path::PathBuf;

use anyhow::{anyhow, Context};
use chrono::Utc;
use serde::Deserialize;
use surrealdb::{
    engine::any::Any,
    sql::{Datetime, Thing},
    Surreal,
};
use tokio::fs;
use torture_parser::parser::{assets::BaseAsset, Parser};

use asset::AssetModel;
use workshop::{WorkshopItemModel, WorkshopItemUpdate};

use crate::{
    collector::masterbundle::MasterBundle,
    toml::{ConfigFile, Workshop},
};

pub struct Importer {
    db: Surreal<Any>,
    config: ConfigFile,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Option<String>,
}

impl Importer {
    pub fn new(db: Surreal<Any>, config: ConfigFile) -> Self {
        Self { db, config }
    }

    // TODO: Multithread this.
    pub async fn import(self) -> anyhow::Result<()> {
        // Create workshop_item record
        let db = &self.db;

        let config = &self.config;
        let workshop = config.workshop.clone();

        let workshop_item = Importer::upsert_workshop_item(db, workshop).await?;
        let workshop_item = workshop_item.unwrap();
        tracing::info!("Created or updated workshop item: {:?}", workshop_item);

        // Parse masterbundle and get paths from it.
        let config_path = self.config.path;
        let bundle = MasterBundle::new(&config_path)?;
        let paths = bundle.get_paths();

        let mut assets: Vec<BaseAsset> = Vec::with_capacity(paths.len());

        for path in paths {
            let mut data_path: PathBuf = config_path.join(path);

            let stem = path
                .file_stem()
                .ok_or_else(|| anyhow!("Failed to get file stem for: {:#?}", path))?;
            let stem = stem
                .to_str()
                .ok_or_else(|| anyhow!("Failed to convert into a &str"))?;
            data_path.push(format!("{}.dat", stem));

            match fs::read_to_string(&data_path).await {
                Ok(content) => {
                    let directory = data_path
                        .parent()
                        .context("Failed to get the parent of data file")?;

                    match BaseAsset::parse(directory, &content) {
                        Ok(asset) => assets.push(asset),
                        Err(err) => tracing::error!("Got error while parsing base asset: {err}"),
                    };
                }
                Err(err) => {
                    tracing::error!(file = ?data_path, "Failed to get content of file: {err}")
                }
            }
        }

        let _ = Importer::handle_assets(db, bundle, workshop_item, assets).await?;

        Ok(())
    }

    /// Insert or update a workshop item
    async fn upsert_workshop_item(
        db: &Surreal<Any>,
        workshop: Workshop,
    ) -> anyhow::Result<Option<WorkshopItemModel>> {
        let item: Option<WorkshopItemModel> = db
            .upsert(("workshop_item", workshop.id))
            .merge(WorkshopItemUpdate {
                authors: workshop.authors,
                last_updated_at: Datetime::from(Utc::now()),
                name: workshop.name,
            })
            .await?;
        Ok(item)
    }

    // TODO: Use bulk insert.
    async fn handle_assets(
        db: &Surreal<Any>,
        bundle: MasterBundle,
        workshop_item: WorkshopItemModel,
        assets: Vec<BaseAsset>,
    ) -> anyhow::Result<()> {
        let models: Vec<AssetModel> = assets
            .into_iter()
            .map(|x| AssetModel::from_base_asset(&workshop_item, x))
            .collect();

        let items: Vec<AssetModel> = db.insert("asset").content(models).await?;
        tracing::info!(length = items.len(), "Inserted assets into database");

        // // Create relations between workshop item and all the assets
        // let records: Vec<_> = items.into_iter().map(|x| x.id.unwrap()).collect();
        // let _ = db
        //     .query("RETURN fn::relate_assets($workshop_item, $assets, $bundle_name);")
        //     .bind(("workshop_item", workshop_item.id)) // Unwrap is fine.
        //     .bind(("assets", records))
        //     .bind(("bundle_name", bundle.name.clone()))
        //     .await?;

        Ok(())
    }
}

impl AssetModel {
    fn from_base_asset(workshop_item: &WorkshopItemModel, asset: BaseAsset) -> Self {
        Self {
            id: Some(Thing::from(("asset", asset.guid.as_str()))),
            name: asset.name,
            description: asset.description,
            guid: asset.guid,
            metadata: None,
            rarity: asset.rarity,
            r#type: asset.r#type,
            non_unique_id: asset.id,
            inserted_at: Datetime::from(Utc::now()),
            workshop_item: workshop_item.id.clone().unwrap(),
        }
    }
}
