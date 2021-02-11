pub mod adapter;

use adapter::ConnectionClient;

#[derive(Debug)]
pub struct Config {
    username: String,
    password: String,
    hostname: String,
    database: String,
    port: i64,
}

impl Config {
    pub fn new(
        username: &str,
        password: &str,
        hostname: &str,
        database: &str,
        port: i64,
    ) -> Config {
        Config {
            username: String::from(username),
            password: String::from(password),
            hostname: String::from(hostname),
            database: String::from(database),
            port,
        }
    }
}

#[derive(Debug)]
enum Action {
    Create,
    Update,
    Delete,
}

#[derive(Debug)]
pub struct Changeset<T> {
    valid: bool,
    data: T,
    action: Action,
}

pub fn connect<T: ConnectionClient>(config: Config) -> Result<T, String> {
    T::connect(config)
}

pub fn cast<T>(schema: T) -> Changeset<T> {
    Changeset {
        valid: true,
        data: schema,
        action: Action::Create,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_database() {}
}
