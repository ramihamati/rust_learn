use std::fmt;
use std::fmt::{Display};
use serde::{Serialize};
use serde_json;

// StateCodeErrorDetails extra details for an error
#[derive(Serialize)]
struct StateCodeErrorDetails {
    // has_associated_key if the error represents a validation error or is
    // related to certain operation the key would hold a name reference to that
    pub(crate) has_associated_key: String,
    pub(crate) associated_key: String,
}

// StateCodeFailure the error which is represented by the code and a message
#[derive(Serialize)]
struct StateCodeFailure {
    pub(crate) error_code: String,
    pub(crate) error_message: String,

    // HasObjectResult is the error has an extra object as detail
    // the objectResult would store that and the typename is used for
    // deserialization
    pub(crate) has_object_result: bool,
    pub(crate) object_result_type_name: String,
    pub(crate) object_result: String, // serialized version
}

#[derive(Serialize)]
struct StateCodeEntry {
    pub(crate) details: StateCodeErrorDetails,
    pub(crate) failures: Vec<StateCodeFailure>,
}

impl StateCodeEntry {
    pub(crate) fn is_errored(&self) -> bool {
        return self.failures.len() > 0;
    }

    pub(crate) fn is_successful(&self) -> bool {
        return self.failures.len() == 0;
    }
}

#[derive(Serialize)]
struct StateCodeResult {
    status: u16,
    entries: Vec<StateCodeEntry>,
}

impl Display for StateCodeResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let conv = serde_json::to_string(&self);
        write!(f, "{}", conv.unwrap())
    }
}
