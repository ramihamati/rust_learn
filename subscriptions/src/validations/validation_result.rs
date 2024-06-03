use serde::Serialize;

#[derive(Serialize, Debug)]
pub(crate) struct ValidationResult {
    pub(crate) is_valid: bool,
    pub(crate) message: &'static str,
}

impl ValidationResult {
    pub(crate) fn valid() -> ValidationResult {
        ValidationResult {
            is_valid: true,
            message: "",
        }
    }

    pub(crate) fn invalid(message: &'static str) -> ValidationResult {
        ValidationResult {
            is_valid: false,
            message,
        }
    }
}
