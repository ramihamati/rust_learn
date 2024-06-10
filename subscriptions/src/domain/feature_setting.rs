use std::fmt::{Display};
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::validations::validation_result::ValidationResult;

#[derive(Debug, Serialize)]
pub struct EntityFeatureOption {
    pub id: String,
    pub label: String,
    pub version : i32,
    pub effective_date : Option<DateTime<Utc>>,
    pub serialized : String
}

pub trait FeatureOptionValidator{
    fn can_validate(option: EntityFeatureOption) -> bool;
    fn validate(option: EntityFeatureOption) -> ValidationResult;
}
pub enum FeatureValue{
    OptionMaxCount (u32),
}
// impl Display for FeatureOption{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", json!({
//             "id" : self.id.to_string(),
//             "name" : self.name
//         }))
//     }
// }
//
