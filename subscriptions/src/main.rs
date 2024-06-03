mod domain;
mod errors;
mod validations;
mod trip_builder_features;

use uuid::Uuid;
use domain::feature_setting::FeatureOption;
use crate::validations::validate::{IValidate, PredicateValidateOption};
use crate::validations::validation_result::ValidationResult;

struct FeatureTravelogueCount {
    name : &'static str,
    id : &'static str
}



fn main() {
    // println!("Hello, world!");
    //
    // let error = ErrorCode {
    //     error_code: "hello".to_string(),
    //     error_message: "bye".to_string(),
    // };
    //
    // println!("{}", error);
    //
    // let id = Uuid::new_v4();
    //
    // let serviceplan = ServicePlan {
    //     name: "hello".to_string(),
    //     id: Uuid::new_v4(),
    //     features: vec![
    //         FeatureOption{
    //             id : Uuid::new_v4(),
    //             name : "name".to_string()
    //         }
    //     ],
    // };

    let option = FeatureOption{
        label : String::from( "no-of-travelogues"),
        id : Uuid::new_v4().to_string().clone().to_string(),
        versions: Vec::new()
    };

    let validator = PredicateValidateOption::<FeatureOption>::New(
        can_validate,
        validate);

    fn can_validate(option: FeatureOption ) -> bool{
        return option.label == "no-of-travelogues"
    }

    fn validate(option : FeatureOption) -> ValidationResult{
        ValidationResult::valid()
    }


    println!("{:?}",validator.validate(option))
}
