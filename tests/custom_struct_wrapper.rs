#[cfg(feature = "serde")]
mod tests {
    use holda::Holda;
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash, Eq, PartialOrd, Ord)]
    struct MyCustomStruct {
        field1: i32,
        field2: String,
    }

    impl fmt::Display for MyCustomStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.field1, self.field2)
        }
    }

    #[derive(Holda)]
    struct MyCustomStructWrapper {
        inner: MyCustomStruct,
    }

    #[test]
    fn test_custom_struct_wrapper() {
        let custom_struct = MyCustomStruct {
            field1: 10,
            field2: "test".to_string(),
        };
        let wrapper = MyCustomStructWrapper::new(custom_struct);
        assert_eq!(wrapper.inner.field1, 10);
        assert_eq!(wrapper.inner.field2, "test");
    }

    #[test]
    fn test_custom_struct_serde() {
        let custom_struct = MyCustomStruct {
            field1: 20,
            field2: "serde_test".to_string(),
        };
        let wrapper = MyCustomStructWrapper::new(custom_struct);
        let serialized = serde_json::to_string(&wrapper).unwrap();
        let deserialized: MyCustomStructWrapper = serde_json::from_str(&serialized).unwrap();

        assert_eq!(wrapper, deserialized);
    }
}
