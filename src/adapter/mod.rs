mod postgres;

pub use self::postgres::Postgres;
use super::Config;

pub trait ConnectionClient {
    fn connect(config: Config) -> Result<Self, String>
    where
        Self: Sized;
}
