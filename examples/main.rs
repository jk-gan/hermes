use hermes::{connect, Adapter, DBConfig};

fn main() {
    let config = DBConfig::new(
        "postgres",
        "password",
        "localhost",
        "hermes",
        5432,
        Adapter::Postgres,
    );

    match connect(config) {
        Ok(_client) => println!("Database is connected successfully"),
        Err(error) => panic!("Error: {}", error),
    }
}
