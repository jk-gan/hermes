mod adapter;

use postgres::{Client, NoTls};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DBConfig {
    dev: DevConfig,
}

#[derive(Debug, Deserialize)]
struct DevConfig {
    username: String,
    password: String,
    hostname: String,
    database: String,
    port: i64,
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

pub fn connect(
    hostname: &str,
    database: &str,
    username: &str,
    password: &str,
) -> Result<Client, String> {
    match Client::connect(
        format!(
            "host={} user={} password={} dbname={}",
            hostname, username, password, database
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
    fn connect_to_database() {
        let connection_result = connect("localhost", "hermes", "postgres", "password");
        assert!(connection_result.is_ok());
    }
}
