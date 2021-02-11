use super::ConnectionAdapter;

struct Postgres {}

impl ConnectionAdapter for Postgres {
    fn connect() -> Self {
        Self {}
    }
}
