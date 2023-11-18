use anyhow::Context;
use blockscout::{Client, Settings};
use blockscout_service_launcher::{self as launcher, launcher::ConfigSettings};
use migration::Migrator;

const SERVICE_NAME: &str = "blockscout-extractor";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    launcher::tracing::init_logs(SERVICE_NAME, &Default::default(), &Default::default())
        .context("tracing initialization")?;

    let settings = Settings::build().context("failed to read config")?;

    let mut connect_options = sea_orm::ConnectOptions::new(&settings.database_url);
    connect_options.sqlx_logging_level(tracing::log::LevelFilter::Debug);
    let db_connection = launcher::database::initialize_postgres::<Migrator>(
        connect_options,
        settings.create_database,
        settings.run_migrations,
    )
    .await?;

    let mut eth_bytecode_db_connect_options =
        sea_orm::ConnectOptions::new(&settings.eth_bytecode_db_database);
    eth_bytecode_db_connect_options.sqlx_logging_level(tracing::log::LevelFilter::Debug);
    let eth_bytecode_db_db_connection = launcher::database::initialize_postgres::<Migrator>(
        eth_bytecode_db_connect_options,
        settings.create_database,
        settings.run_migrations,
    )
    .await?;

    let client = Client::try_new(
        db_connection,
        eth_bytecode_db_db_connection,
        settings.eth_bytecode_db_url,
    )?;

    tracing::info!("importing contract addresses started");
    let processed = client.import_contract_addresses().await?;
    tracing::info!("importing contract addresses finished. Total processed contracts={processed}");

    let mut handles = Vec::with_capacity(settings.n_threads);
    for _ in 0..settings.n_threads {
        let client = client.clone();
        let handle = tokio::spawn(client.verify_contracts());
        handles.push(handle);
    }
    for result in futures::future::join_all(handles).await {
        result.context("join handle")?.context("verify contracts")?;
    }

    Ok(())
}
