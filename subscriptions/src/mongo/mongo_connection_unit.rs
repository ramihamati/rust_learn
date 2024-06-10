use std::fmt::Error;
use std::ops::Deref;
use chrono::Duration;
use mongodb::{Client, Collection};
use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use crate::mongo::mongo_connection_options::{MongoConnectionSettings, MongoConnectionSettingsProvider};

#[derive(Debug)]
pub struct MongoConnectionUnit {
    connection_settings_provider: Box<dyn MongoConnectionSettingsProvider>,
    options: MongoConnectionSettings,
    client: Client,
}

impl MongoConnectionUnit {
    pub async fn new(provider: Box<dyn MongoConnectionSettingsProvider>) -> MongoConnectionUnit {
        let provided_settings = provider.get();
        let settings = Self::initialize_defaults(provided_settings);

        MongoConnectionUnit {
            options: settings.clone(),
            connection_settings_provider: provider,
            client: Client::with_uri_str(settings.connection_string).await.unwrap(),
        }
    }

    fn initialize_defaults(connection: MongoConnectionSettings) -> MongoConnectionSettings {
        MongoConnectionSettings {
            connection_string: connection.connection_string,
            retry_count: Some(connection.retry_count.unwrap_or_else(|| 3)),
            connection_timeout: Some(connection.connection_timeout.unwrap_or_else(|| Duration::seconds(2))),
            sleep_between_retries: Some(connection.sleep_between_retries.unwrap_or_else(|| Duration::seconds(3))),
        }
    }

    fn get_database(&self, name: &str) -> mongodb::Database {
        self.client.database(name)
    }

    async fn get_or_create_collection<T>(&self, collection_name: &str, databaseName: &str) -> Result<Collection<T>, mongodb::error::Error> {
        let retry_strategy = ExponentialBackoff::from_millis(100)
            .map(jitter)
            .take(3);

        let result = Retry::spawn(retry_strategy, || {
            self._create_collection(collection_name, databaseName)
        }).await?;

        Result::Ok(result)
    }

    async fn _create_collection<T>(&self, collection_name: &str, databaseName: &str) -> Result<Collection<T>, mongodb::error::Error> {
        let db = self.get_database(databaseName);

        let db_collection_names = db.list_collection_names(None).await?;

        let collection_exists = db_collection_names.iter()
            .any(|name| {
                return name.as_str() == collection_name;
            });

        if collection_exists == false
        {
            db.create_collection(collection_name.to_string(), None).await?;
        }

        return Result::Ok(db.collection(collection_name));
    }
}