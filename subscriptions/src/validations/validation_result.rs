use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub message: &'static str,
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
