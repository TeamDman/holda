mod tests {
    use holda::Holda;
    use uuid::Uuid;

    #[derive(Holda)]
    struct MyUuidWrapper {
        inner: Uuid,
    }

    #[test]
    fn test_uuid_wrapper() {
        let uuid = Uuid::new_v4();
        let wrapper = MyUuidWrapper::new(uuid);

        assert_eq!(*wrapper, uuid);
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde_json;

        #[test]
        fn test_uuid_serde() {
            let uuid = Uuid::new_v4();
            let wrapper = MyUuidWrapper::new(uuid);

            let serialized = serde_json::to_string(&wrapper).unwrap();
            let deserialized: MyUuidWrapper = serde_json::from_str(&serialized).unwrap();

            assert_eq!(wrapper, deserialized);
        }
    }
}
