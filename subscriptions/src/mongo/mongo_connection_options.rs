use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
pub struct MongoConnectionSettings{
    pub connection_string: String,
    pub connection_timeout: Option<chrono::Duration>,
    pub retry_count : Option<i8>,
    pub sleep_between_retries: Option<chrono::Duration>,
}


pub trait MongoConnectionSettingsProvider {
     fn get(&self) -> MongoConnectionSettings;
}

impl Debug for dyn MongoConnectionSettingsProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "trait")
    }
}