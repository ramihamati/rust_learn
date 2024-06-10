mod domain;
mod errors;
mod validations;
mod trip_builder_features;
mod mongo;
mod configurations;

use chrono::{Utc};
use uuid::Uuid;
use crate::configurations::configuration_mongo::ConfigurationMongo;
use crate::domain::feature_setting::EntityFeatureOption;
use crate::mongo::mongo_connection_unit::MongoConnectionUnit;
use crate::validations::validate::{IValidate};

struct FeatureTravelogueCount {
    name: &'static str,
    id: &'static str,
}

#[tokio::main]
async fn main() {
    let option = EntityFeatureOption {
        label: String::from("no-of-travelogues"),
        id: Uuid::new_v4().to_string().clone().to_string(),
        version: 0,
        effective_date: Some(Utc::now()),
        serialized: "".to_string(),
    };

    let provider = ConfigurationMongo {
        connection_string: "mongodb://100.103.106.35:27017/?readPreference=primary".to_string(),
        connection_timeout: None,
        sleep_between_retries: None,
        retry_count: None,
    };

    let connection_unit = MongoConnectionUnit::new(Box::new(provider)).await;

    println!("{:?}", connection_unit)
}
