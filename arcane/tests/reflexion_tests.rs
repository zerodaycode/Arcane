//! Tests for the reflexion API


#[cfg(test)]
mod reflexion_tests {
    use arcane::reflexion::*;
    
    /// Mock type to work with the tests
    #[derive(StructInfo)]
    #[allow(dead_code)]
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

    #[test]
    fn check_struct_info() {
        let mock = ReflectiveMock { 
            id: 1,
            username: "Pyzyryab".to_string()
        };

        let si = mock.get_info();

        assert_eq!(si.name, "ReflectiveMock");

        let first_field = si.fields.clone();
        let ff = first_field.get(0).unwrap();
        assert_eq!(ff.name, "id");
        assert_eq!(ff.typ, "i32");
        assert!(ff.attrs.is_empty());

        let second_field = si.fields.clone();
        let sf = second_field.get(1).unwrap();
        assert_eq!(sf.name, "username");
        assert_eq!(sf.typ, "String");
        assert!(sf.attrs.is_empty());

        println!("SI: {:?}", si.name);
        println!("Fields: {:?}", si.fields);
        println!("Attrs: {:?}", si.attrs);
    }
}