use hermes::{adapter::Postgres, connect, Config};

fn main() {
    let pg_config = Config::new("postgres", "password", "localhost", "hermes", 5432);

    match connect::<Postgres>(pg_config) {
        Ok(_client) => println!("Database is connected successfully"),
        Err(error) => panic!("Error: {}", error),
    }
}
