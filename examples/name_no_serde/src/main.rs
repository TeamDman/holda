use holda::StringHolda;

#[derive(StringHolda)]
struct UserName {
    inner: String,
}

fn main() {
    let name = "bruh";
    let user_name: UserName = name.parse().unwrap();
    assert_eq!(*user_name, name);
    let user_name = UserName::from(name);
    assert_eq!(*user_name, name);
}
