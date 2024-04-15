use tokio_postgres::{Client, Config, Error, Row};

pub async fn validate_user(client: &Client, name: &str) -> Result<Row, Error> {
    let query = "SELECT usename from pg_catalog.pg_user where usename=$1;".to_string();
    client.query_one(&query, &[&name]).await
}

#[allow(dead_code)]
pub async fn update_user_validatity(
    client: &tokio_postgres::Client,
    name: &str,
    pwd: &str,
    expire_at: &str,
) -> Result<u64, tokio_postgres::Error> {
    let query = build_query(name, pwd, expire_at);
    client.execute(&query, &[]).await
}

fn build_query(uname: &str, pwd: &str, expire_at: &str) -> String {
    let mut query = String::new();
    query.push_str("ALTER USER ");
    query.push_str(uname);
    query.push_str(" ");
    query.push_str("WITH PASSWORD ");
    query.push_str("'");
    query.push_str(pwd);
    query.push_str("'");
    query.push_str(" ");
    query.push_str("VALID UNTIL ");
    query.push_str("'");
    query.push_str(expire_at);
    query.push_str("';");
    query
}

pub fn create_config() -> Config {
    let user = env!("DB_USER");
    let pwd = env!("DB_PASSWORD");
    let dbname = env!("DB_NAME");
    let host = env!("DB_HOST");
    let port = env!("DB_PORT").parse::<u16>();

    let mut pg_config = tokio_postgres::Config::new();
    let port_value = port.unwrap();
    pg_config.port(port_value);
    pg_config.password(pwd);
    pg_config.user(user);
    pg_config.dbname(dbname);
    pg_config.host(host);
    pg_config.ssl_mode(postgres::config::SslMode::Disable);
    pg_config
}
