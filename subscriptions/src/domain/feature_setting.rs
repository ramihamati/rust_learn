use serde::Serialize;
use std::fmt::{Display};
use crate::validations::validate::PredicateValidateOption;

pub struct FeatureOption {
    pub id: String,
    pub label: String,
    pub versions : Vec<FeatureOptionVersion>
}

pub struct FeatureOptionVersion {
    pub version: u32,
    pub validator : PredicateValidateOption<FeatureValue>
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
