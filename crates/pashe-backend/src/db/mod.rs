use clickhouse::Client;

pub fn get_client(url: &str, user: &str, password: &str, database: &str) -> clickhouse::Client {
    Client::default()
        .with_url(url)
        .with_user(user)
        .with_password(password)
        .with_database(database)
}
