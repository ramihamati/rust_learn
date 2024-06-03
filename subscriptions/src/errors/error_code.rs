use std::fmt::{Display, Formatter};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub(crate) struct ErrorCode {
    pub(crate) error_code: String,
    pub(crate) error_message: String,
}

impl Display for ErrorCode{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", json!(self))
    }
}
