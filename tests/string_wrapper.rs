use holda::StringHolda;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::str::FromStr;

#[derive(StringHolda)]
struct MyStringWrapper {
    inner: String,
}

#[test]
fn test_new() {
    let wrapper = MyStringWrapper::new("test");
    assert_eq!(wrapper.inner.as_str(), "test");
}

#[test]
fn test_as_ref() {
    let wrapper = MyStringWrapper::new("test");
    let as_ref: &str = wrapper.as_ref();
    assert_eq!(as_ref, "test");
}

#[test]
fn test_from() {
    let wrapper = MyStringWrapper::from("test");
    assert_eq!(wrapper.inner.as_str(), "test");
}

#[test]
fn test_into() {
    let wrapper = MyStringWrapper::new("test");
    let inner: String = wrapper.into();
    assert_eq!(inner.as_str(), "test");
}

#[test]
fn test_deref() {
    let wrapper = MyStringWrapper::new("test");
    assert_eq!(*wrapper, "test");
}

#[test]
fn test_deref_mut() {
    let mut wrapper = MyStringWrapper::new("test");
    *wrapper = "new_test".into();
    assert_eq!(*wrapper, "new_test");
}

#[test]
fn test_display() {
    let wrapper = MyStringWrapper::new("test");
    assert_eq!(wrapper.to_string(), "test");
}

#[test]
fn test_from_str() {
    let wrapper = MyStringWrapper::from_str("test").unwrap();
    assert_eq!(*wrapper, "test");
}

#[test]
fn test_partial_eq() {
    let wrapper1 = MyStringWrapper::new("test");
    let wrapper2 = MyStringWrapper::new("test");
    assert_eq!(wrapper1, wrapper2);

    let wrapper3 = MyStringWrapper::new("test3");
    assert_ne!(wrapper1, wrapper3);
}

#[test]
fn test_eq() {
    let wrapper1 = MyStringWrapper::new("test");
    let wrapper2 = MyStringWrapper::new("test");
    assert_eq!(wrapper1, wrapper2);
}

#[test]
fn test_partial_ord() {
    let wrapper1 = MyStringWrapper::new("test1");
    let wrapper2 = MyStringWrapper::new("test2");
    assert_eq!(wrapper1.partial_cmp(&wrapper2), Some(Ordering::Less));
}

#[test]
fn test_ord() {
    let wrapper1 = MyStringWrapper::new("test1");
    let wrapper2 = MyStringWrapper::new("test2");
    assert_eq!(wrapper1.cmp(&wrapper2), Ordering::Less);
}

#[test]
fn test_hash() {
    use std::collections::hash_map::DefaultHasher;

    let wrapper1 = MyStringWrapper::new("test");
    let wrapper2 = MyStringWrapper::new("test");

    let mut hasher1 = DefaultHasher::new();
    wrapper1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    wrapper2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_clone() {
    let wrapper1 = MyStringWrapper::new("test");
    let wrapper2 = wrapper1.clone();
    assert_eq!(wrapper1, wrapper2);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    use serde_json;

    let wrapper = MyStringWrapper::new("test");
    let serialized = serde_json::to_string(&wrapper).unwrap();
    let deserialized: MyStringWrapper = serde_json::from_str(&serialized).unwrap();
    assert_eq!(wrapper, deserialized);
}
