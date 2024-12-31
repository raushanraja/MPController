use dotenvy::dotenv;

pub fn config_env() -> String {
    dotenv().ok();

    match std::env::var("RUST_LOG") {
        Ok(loglevel) => {
            println!("Setting level to: {:?}", loglevel);
        }
        Err(_) => std::env::set_var("RUST_LOG", "info"),
    };

    let port = match std::env::var("PORT") {
        Ok(val) => val,
        Err(_) => "8080".to_string(),
    };

    env_logger::init();
    port
}
