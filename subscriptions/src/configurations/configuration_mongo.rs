use crate::mongo::mongo_connection_options::{MongoConnectionSettings, MongoConnectionSettingsProvider};

pub struct ConfigurationMongo
{
    pub connection_string: String,
    pub connection_timeout: Option<chrono::Duration>,
    pub retry_count: Option<i8>,
    pub sleep_between_retries: Option<chrono::Duration>,
}

impl MongoConnectionSettingsProvider for ConfigurationMongo {
    fn get(&self) -> MongoConnectionSettings {
        MongoConnectionSettings {
            retry_count: self.retry_count,
            sleep_between_retries: self.sleep_between_retries,
            connection_timeout: self.connection_timeout,
            connection_string: self.connection_string.clone(),
        }
    }
}