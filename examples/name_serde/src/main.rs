use holda::StringHolda;
use serde::Deserialize;
use serde::Serialize;

#[derive(StringHolda)]
struct UserName {
    inner: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash, Eq)]
struct Payload {
    name: UserName,
}

fn main() {
    let data = r#"
    {
        "name": "Bruh"
    }
    "#;
    let payload: Payload = serde_json::from_str(data).unwrap();
    println!("{:?}", payload);
    assert_eq!(*payload.name, "Bruh");
}
