mod auth;
mod handlers;
mod responses;

use crate::application::http::auth::AuthenticationLayer;
use crate::application::http::handlers::create_role::create_role;
use crate::application::http::handlers::get_roles::get_roles;
use crate::domain::member::ports::MemberService;
use crate::domain::role::ports::RoleService;
use crate::env::Env;
use anyhow::Context;
use axum::routing::{get, post};
use axum::Extension;
use std::sync::Arc;
use tokio::net;
use tracing::{info, info_span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Debug, Clone)]
struct AppState<R, M>
where
    R: RoleService,
    M: MemberService,
{
    role_service: Arc<R>,
    member_service: Arc<M>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<R, M>(
        config: HttpServerConfig,
        env: Arc<Env>,
        role_service: Arc<R>,
        member_service: Arc<M>,
    ) -> anyhow::Result<Self>
    where
        R: RoleService,
        M: MemberService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState {
            role_service,
            member_service,
        };

        let auth_layer = AuthenticationLayer::new(env.auth_service_url.clone());

        let router = axum::Router::new()
            .nest("", api_routes())
            .layer(trace_layer)
            .layer(auth_layer)
            .layer(Extension(Arc::clone(&state.role_service)))
            .layer(Extension(Arc::clone(&state.member_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );
        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}

fn api_routes<R, M>() -> axum::Router<AppState<R, M>>
where
    R: RoleService,
    M: MemberService,
{
    axum::Router::new()
        .route("/guilds/:guild_id/roles", get(get_roles::<R>))
        .route("/guilds/:guild_id/roles", post(create_role::<R>))
}
