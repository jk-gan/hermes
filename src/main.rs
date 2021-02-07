use hermes::{cast, connect};

#[derive(Debug)]
struct User {
    name: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}

fn main() {
    connect();
    let changeset = cast(User::default());
    println!("{:?}", changeset);
}
