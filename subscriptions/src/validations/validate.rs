use crate::validations::validation_result::ValidationResult;

pub(crate) trait IValidate<T> {
    fn validate(&self, option: T) -> ValidationResult;
}

pub(crate) struct PredicateValidateOption<T> {
    validator: fn(feature_option: T) -> ValidationResult,
}

impl<T> PredicateValidateOption<T> {
    pub(crate) fn New(
        can_validate: fn(feature_option: T) -> bool,
        validator: fn(feature_option: T) -> ValidationResult) -> PredicateValidateOption <T> {
        return PredicateValidateOption {
            validator: validator.clone(),
        };
    }
}

impl<T> IValidate<T> for PredicateValidateOption<T>{
    fn validate(&self, option: T) -> ValidationResult {
        (self.validator)(option)
    }
}