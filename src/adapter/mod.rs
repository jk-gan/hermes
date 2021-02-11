mod postgres;

#[derive(Debug)]
pub enum Adapter {
    Postgres,
    MySQL,
    Redis,
}

trait ConnectionAdapter {
    fn connect() -> Self;
}
