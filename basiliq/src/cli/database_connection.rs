use super::*;

macro_rules! fill_database_url_option {
    ($cli_matches:expr, $opt:ident, $opt_name:ident) => {
        match $cli_matches.value_of(concat!("db", stringify!($opt_name))) {
            Some(x) => $opt.$opt_name(x),
            None => $opt,
        }
    };
}

macro_rules! fill_database_url_option_number {
    ($cli_matches:expr, $opt:ident, $opt_name:ident, $type_:ty) => {
        match $cli_matches.value_of(concat!("db", stringify!($opt_name))) {
            Some(x) => $opt.$opt_name(
                <$type_>::from_str_radix(x, 10).expect("A valid, positive, natural number"),
            ),
            None => $opt,
        }
    };
}

pub fn gen_database_url(cli_matches: &ArgMatches) -> PgConnectOptions {
    let env_database_url = std::env::var("DATABASE_URL")
        .ok()
        .map(|x| PgConnectOptions::from_str(x.as_str()))
        .transpose()
        .expect("Failed to parse database url from environment");
    let mut base_database_url = env_database_url.unwrap_or_default();
    base_database_url = fill_database_url_option!(cli_matches, base_database_url, host);
    base_database_url = fill_database_url_option!(cli_matches, base_database_url, database);
    base_database_url = fill_database_url_option!(cli_matches, base_database_url, username);
    base_database_url = fill_database_url_option!(cli_matches, base_database_url, password);
    base_database_url = fill_database_url_option!(cli_matches, base_database_url, ssl_root_cert);
    base_database_url = fill_database_url_option_number!(cli_matches, base_database_url, port, u16);
    base_database_url = fill_database_url_option_number!(
        cli_matches,
        base_database_url,
        statement_cache_capacity,
        usize
    );
    base_database_url = match cli_matches.value_of("dbssl_mode") {
        Some(x) => base_database_url.ssl_mode(sqlx::postgres::PgSslMode::from_str(x).unwrap()),
        None => base_database_url,
    };
    base_database_url = match cli_matches.value_of("dbapp_name") {
        Some(x) => base_database_url.application_name(format!("basiliq_{}", x).as_str()),
        None => base_database_url,
    };
    base_database_url
}

pub fn gen_db_connection_options(cli_matches: &ArgMatches) -> BasiliqDbConnectionOption {
    let connection_option = gen_database_url(cli_matches);
    let pool_max_connections = cli_matches
        .value_of("dbconn_nb")
        .map(|x| usize::from_str_radix(x, 10).expect("A valid, positive, natural number"));

    BasiliqDbConnectionOption {
        connection_option,
        pool_max_connections,
    }
}
