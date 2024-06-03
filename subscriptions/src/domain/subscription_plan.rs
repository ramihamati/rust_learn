use std::fmt::{Display, Formatter};
use serde::Serialize;
use uuid::Uuid;
use crate::domain::feature_setting::FeatureOption;

pub(crate) struct ServicePlan {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) features: Vec<FeatureOption>,
}

// impl Display for ServicePlan {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut ids = Vec::<String>::new();
//
//         for x in &self.features {
//             ids.push(x.id.to_string())
//         }
//
//         write!(f, "{}", json!({
//             "id" : self.id.to_string(),
//             "name" : self.name,
//
//             "features" : sized
//         }))
//     }
// }
