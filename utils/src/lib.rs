use std::{env, str::FromStr};

use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgConnection};

pub async fn setup_and_get_db_conn() -> PgConnection {
    dotenvy::dotenv().unwrap();

    PgConnectOptions::from_str(&env::var("DATABASE_URL").expect(
        "É necessária uma variável de ambiente \"DATABASE_URL\" para inicializar a aplicação.",
    ))
    .unwrap()
    .connect()
    .await
    .unwrap()
}
