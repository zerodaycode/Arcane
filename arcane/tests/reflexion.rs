//! Tests for the reflexion API


#[cfg(test)]
mod reflexion_tests {
    use arcane::reflexion::*;

    /// Mock type to work with the tests
    #[derive(Reflexion)]
    struct ReflectiveMock;

    #[test]
    fn get_struct_name() {
        let mock = ReflectiveMock;
        assert_eq!(mock.get_struct_name(), "ReflectiveMock");
    }
}