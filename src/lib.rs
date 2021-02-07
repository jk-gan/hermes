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

pub fn connect() {
    let path = Path::new("./src/database.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            println!("{} contains:\n{}", display, s);
            let db_config: DBConfig = toml::from_str(&s).unwrap();
            println!("{:?}", db_config);
            match Client::connect(
                format!(
                    "host={} user={} password={}",
                    db_config.dev.hostname, db_config.dev.username, db_config.dev.password
                )
                .as_ref(),
                NoTls,
            ) {
                Ok(_client) => {
                    println!("connected successfully");
                }
                Err(err) => println!("{}", err),
            }
        }
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
