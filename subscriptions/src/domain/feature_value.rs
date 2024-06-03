use std::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct FeatureValue {
    pub(crate) value_type: String,
    pub(crate) value: Vec<u8>,
}

impl Display for FeatureValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", serde_json::to_string(self))
    }
}
