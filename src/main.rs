use std::env;

use anyhow::Context;
use clap::Parser;
use surrealdb::{engine::any::Any, Surreal};
use torture_importer::{importer::Importer, toml::ConfigFile};

mod cli;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    tracing_subscriber::fmt::init();
    tracing::info!("Starting torture asset importer.");

    dotenvy::dotenv().ok();

    let db = initialize_database().await?;

    let config = ConfigFile::from_file(args.config)?;
    println!("{:#?}", config);

    let importer = Importer::new(db, config);

    match importer.import().await {
        Ok(_) => tracing::info!("Imported!"),
        Err(err) => tracing::error!("Failed to import, {err}"),
    };

    Ok(())
}

async fn initialize_database() -> anyhow::Result<Surreal<Any>> {
    use database::{ConnectionOptions, Database};
    use surrealdb::opt::auth::Root;

    let surreal_endpoint =
        env::var("SURREAL_ENDPOINT").context("SURREAL_ENDPOINT is not set in .env file")?;
    let surreal_user = env::var("SURREAL_USER").context("SURREAL_USER is not set in .env file")?;
    let surreal_password =
        env::var("SURREAL_PASSWORD").context("SURREAL_PASSWORD is not set in .env file")?;
    let surreal_namespace =
        env::var("SURREAL_NAMESPACE").context("SURREAL_NAMESPACE is not set in .env file")?;
    let surreal_database =
        env::var("SURREAL_DATABASE").context("SURREAL_DATABASE is not set in .env file")?;

    let connect_options = ConnectionOptions {
        namespace: &surreal_namespace,
        database: &surreal_database,
        credentials: Root {
            username: &surreal_user,
            password: &surreal_password,
        },
    };

    let db = Database::connect(&surreal_endpoint, &connect_options).await?;

    Ok(db)
}
