use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "DOMAIN",
            dotenv::var("DOMAIN").expect("Expected DOMAIN to be set in env!"),
        );
        map.insert(
            "ADDR",
            dotenv::var("ADDR").expect("Expected ADDR to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected PORT to be set in env!"),
        );
        map.insert(
            "LOG_LEVEL",
            dotenv::var("LOG_LEVEL").expect("Expected LOG_LEVEL to be set in env!"),
        );

        map.insert(
            "GQL_PROT",
            dotenv::var("GQL_PROT").expect("Expected GQL_PROT to be set in env!"),
        );
        map.insert(
            "GQL_ADDR",
            dotenv::var("GQL_ADDR").expect("Expected GQL_ADDR to be set in env!"),
        );
        map.insert(
            "GQL_PORT",
            dotenv::var("GQL_PORT").expect("Expected GQL_PORT to be set in env!"),
        );
        map.insert(
            "GQL_URI",
            dotenv::var("GQL_URI").expect("Expected GQL_URI to be set in env!"),
        );
        map.insert(
            "GQL_VER",
            dotenv::var("GQL_VER").expect("Expected GQL_VER to be set in env!"),
        );
        map.insert(
            "GIQL_VER",
            dotenv::var("GIQL_VER").expect("Expected GIQL_VER to be set in env!"),
        );

        map.insert(
            "EMAIL_SMTP",
            dotenv::var("EMAIL_SMTP").expect("Expected EMAIL_SMTP to be set in env!"),
        );
        map.insert(
            "EMAIL_FROM",
            dotenv::var("EMAIL_FROM").expect("Expected EMAIL_FROM to be set in env!"),
        );
        map.insert(
            "EMAIL_USERNAME",
            dotenv::var("EMAIL_USERNAME").expect("Expected EMAIL_USERNAME to be set in env!"),
        );
        map.insert(
            "EMAIL_PASSWORD",
            dotenv::var("EMAIL_PASSWORD").expect("Expected EMAIL_PASSWORD to be set in env!"),
        );

        map
    };
}
