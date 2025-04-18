mod tests {
    use holda::Holda;

    #[derive(Holda)]
    #[holda(NoDisplay, NoEq, NoOrd, NoHash)]
    struct MyUnitWrapper {
        inner: (),
    }

    #[test]
    fn test_unit_wrapper() {
        let wrapper = MyUnitWrapper::new(());
        assert_eq!(wrapper.inner, ());
    }
}
