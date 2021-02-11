use super::ConnectionClient;
use crate::Config;
use postgres::{Client, NoTls};

pub struct Postgres {
    client: Client,
}

impl ConnectionClient for Postgres {
    fn connect(config: Config) -> Result<Self, String> {
        match Client::connect(
            format!(
                "host={} user={} password={} dbname={}",
                config.hostname, config.username, config.password, config.database
            )
            .as_ref(),
            NoTls,
        ) {
            Ok(client) => {
                let postgres = Postgres { client };
                Ok(postgres)
            }
            Err(_) => Err(String::from("Can't connect to database")),
        }
    }
}
