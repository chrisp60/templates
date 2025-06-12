fn main() -> app::Result {
    tracing_subscriber::fmt().with_env_filter("app=info").init();
    tokio::runtime::Runtime::new()?.block_on(app::run())
}
