use infrastructure::PgPool;

#[derive(Clone,Debug)]
pub struct AppState {
    pub db: PgPool,
}