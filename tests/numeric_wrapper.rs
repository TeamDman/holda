mod tests {
    use holda::Holda;

    #[derive(Holda)]
    #[holda(NoEq, NoOrd, NoHash)]
    struct MyI32Wrapper {
        inner: i32,
    }

    #[derive(Holda)]
    #[holda(NoDisplay, NoEq, NoOrd, NoHash)]
    struct MyF64Wrapper {
        inner: f64,
    }

    #[test]
    fn test_i32_wrapper() {
        let wrapper = MyI32Wrapper::new(42);
        assert_eq!(*wrapper, 42);
    }

    #[test]
    fn test_f64_wrapper() {
        let wrapper = MyF64Wrapper::new(3.14);
        assert_eq!(*wrapper, 3.14);
    }
}
