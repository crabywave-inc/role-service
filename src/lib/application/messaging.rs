use std::sync::Arc;
use anyhow::Result;
use crate::application::ports::messaging_ports::{MessagingPort, MessagingTypeImpl};
use crate::domain::member::events::MemberCreatedEvent;
use crate::domain::member::ports::MemberService;
use crate::domain::role::ports::RoleService;

pub async fn start_subscriptions<R, M>(
    messaging: Arc<MessagingTypeImpl>,
    role_service: Arc<R>,
    member_service: Arc<M>,
) -> Result<()>
where
    R: RoleService,
    M: MemberService,
{
    let messaging = Arc::clone(&messaging);

    messaging
        .subscribe("members-created-role", {
            let member_service = Arc::clone(&member_service);

            move |msg: MemberCreatedEvent| {
                let member_service = Arc::clone(&member_service);
                async move {
                    member_service.create(msg).await?;
                    Ok(())
                }
            }
        })
        .await?;

    Ok(())
}