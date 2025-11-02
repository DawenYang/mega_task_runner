use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
};
use anyhow::Ok;
use axum::Router;
use redis::{aio::ConnectionManager, Client};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis: ConnectionManager,
    pub email_client: EmailClient,
    pub base_url: ApplicationBaseUrl,
    pub hmac_secret: HmacSecret,
}

pub struct Application {
    port: u16,
    router: Router,
    listener: TcpListener,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr().unwrap().port();

        let router = build_router(
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_url,
        )
        .await?;

        Ok(Self {
            port,
            router,
            listener,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub SecretString);

fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

async fn build_router(
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: SecretString,
    redis_url: SecretString,
) -> Result<Router, anyhow::Error> {
    use axum::routing::get;

    let redis_client = Client::open(redis_url.expose_secret().to_string())?;
    let redis = ConnectionManager::new(redis_client).await?;

    let app_state = AppState {
        db_pool,
        redis,
        email_client,
        base_url: ApplicationBaseUrl(base_url),
        hmac_secret: HmacSecret(hmac_secret),
    };

    let router = Router::new()
        .route("/health", get(health_check))
        .with_state(app_state);
    Ok(router)
}

async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}
