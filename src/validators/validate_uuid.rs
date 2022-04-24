use regex::Regex;


/**
    Verifies if a given uuid string is valid or not
    @param uuid: the uuid to validate

    disclaimer: uuid v2 were not tested due to lack of examples, anyway you're strongly advised
    not to use v2
**/
pub fn validate_uuid(uuid: &String) -> bool {
    let uuid_regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
    uuid_regex.is_match(uuid)
}

#[cfg(test)]
mod tests {
    use crate::validate_uuid;

    #[test]
    fn empty_returns_false(){
        let empty = "".to_string();
        assert!(!validate_uuid(&empty))
    }

    #[test]
    fn random_suite_returns_false(){
        let uuid = "fdsylkfhnoilawsdt77aseodilrftq04o5l34r".to_string();
        assert!(!validate_uuid(&uuid))
    }

    #[test]
    fn v1_returns_true(){
        let uuid = "0073fbbe-c3ac-11ec-9d64-0242ac120002".to_string();
        assert!(validate_uuid(&uuid))
    }

    #[test]
    fn v3_returns_true(){
        let uuid = "5961fe25-14d1-3811-8155-c9c705a0bfc8".to_string();
        assert!(validate_uuid(&uuid))
    }

    #[test]
    fn v4_returns_true(){
        let uuid = "bb28ccfe-4cce-4015-b60e-04796f662b2a".to_string();
        assert!(validate_uuid(&uuid))
    }

    #[test]
    fn v5_returns_true(){
        let uuid = "1c7714fb-1ac4-5861-8acb-86898db3e850".to_string();
        assert!(validate_uuid(&uuid))
    }
}
