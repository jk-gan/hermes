mod adapter;

pub use adapter::Adapter;
use postgres::{Client, NoTls};

#[derive(Debug)]
pub struct DBConfig {
    username: String,
    password: String,
    hostname: String,
    database: String,
    port: i64,
    adapter: Adapter,
}

impl DBConfig {
    pub fn new(
        username: &str,
        password: &str,
        hostname: &str,
        database: &str,
        port: i64,
        adapter: Adapter,
    ) -> DBConfig {
        DBConfig {
            username: String::from(username),
            password: String::from(password),
            hostname: String::from(hostname),
            database: String::from(database),
            port,
            adapter,
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

pub fn connect(db_config: DBConfig) -> Result<Client, String> {
    match Client::connect(
        format!(
            "host={} user={} password={} dbname={}",
            db_config.hostname, db_config.username, db_config.password, db_config.database
        )
        .as_ref(),
        NoTls,
    ) {
        Ok(client) => Ok(client),
        Err(_) => Err(String::from("Can't connect to database")),
    }
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
