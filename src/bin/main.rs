use clap::Parser;

use role::application::http::{HttpServer, HttpServerConfig};
use role::application::messaging::start_subscriptions;
use role::application::ports::messaging_ports::{MessagingType, MessagingTypeImpl};
use role::domain::member::services::MemberServiceImpl;
use role::domain::role::services::RoleServiceImpl;
use role::env::Env;
use role::infrastructure::db::firestore::Firestore;
use role::infrastructure::member::db::firestore_member_repository::FirestoreMemberRepository;
use role::infrastructure::role::db::firestore_role_repository::FirestoreRoleRepository;
use std::sync::Arc;
use role::domain::member::ports::MemberRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let messaging_port =
        Arc::new(MessagingTypeImpl::new(&MessagingType::PubSub, Arc::clone(&env)).await?);

    let firestore = Arc::new(Firestore::new(Arc::clone(&env)).await?);

    let role_repository = FirestoreRoleRepository::new(Arc::clone(&firestore));
    let role_service = Arc::new(RoleServiceImpl::new(role_repository));

    let member_repository = FirestoreMemberRepository::new(Arc::clone(&firestore));
    let member_service = Arc::new(MemberServiceImpl::new(member_repository));


    start_subscriptions(
        Arc::clone(&messaging_port),
        Arc::clone(&role_service),
        Arc::clone(&member_service),
    )
    .await?;

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(
        server_config,
        Arc::clone(&env),
        Arc::clone(&role_service),
        Arc::clone(&member_service),
    )
    .await?;

    http_server.run().await
}
