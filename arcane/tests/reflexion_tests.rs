//! Tests for the reflexion API


#[cfg(test)]
mod reflexion_tests {
    use arcane::reflexion::*;
    use std::collections::HashMap;
    
    /// Mock type to work with the tests
    #[derive(Reflexion)]
    struct ReflectiveMock {
        pub id: i32,
        pub username: String
    }

    #[test]
    fn get_struct_name() {
        let mock = ReflectiveMock { 
            id: 1,
            username: "Pyzyryab".to_string()
        };
        assert_eq!(mock.get_struct_name(), "ReflectiveMock");
    }

    #[test]
    fn check_struct_field_type() {
        let mock = ReflectiveMock { 
            id: 1,
            username: "Pyzyryab".to_string()
        };

        let hm = mock.get_stuct_fields();
        
        assert!(hm.contains_key("id"));
        assert!(hm.contains_key("username"));
        assert!(!hm.contains_key("random_field"));

        assert_eq!(hm.get("id").unwrap(), &"i32");
        assert_eq!(hm.get("username").unwrap(), &"String");
        assert_ne!(hm.get("username").unwrap(), &"i32");
    }
}