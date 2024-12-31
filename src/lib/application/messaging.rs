use crate::application::ports::messaging_ports::{MessagingPort, MessagingTypeImpl};
use crate::domain::member::events::{MemberCreatedEvent, MemberRoleAddedEvent};
use crate::domain::member::ports::MemberService;
use crate::domain::role::ports::role::RoleService;
use anyhow::Result;
use std::sync::Arc;

pub async fn start_subscriptions<R, M>(
    messaging: Arc<MessagingTypeImpl>,
    _role_service: Arc<R>,
    member_service: Arc<M>,
) -> Result<()>
where
    R: RoleService,
    M: MemberService,
{
    let messaging = Arc::clone(&messaging);

    messaging
        // members -> member @TODO: Change this to member-created
        .subscribe("member-created-role", {
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

    messaging
        .subscribe("member-roles-added-role", {
            let member_service = Arc::clone(&member_service);

            move |msg: MemberRoleAddedEvent| {
                let member_service = Arc::clone(&member_service);
                async move {
                    member_service
                        .add_role(&msg.member_id, &msg.role_id)
                        .await?;
                    Ok(())
                }
            }
        })
        .await?;

    messaging
        .subscribe("member-roles-removed-role", {
            let member_service = Arc::clone(&member_service);

            move |msg: MemberRoleAddedEvent| {
                let member_service = Arc::clone(&member_service);
                async move {
                    member_service
                        .remove_role(&msg.member_id, &msg.role_id)
                        .await?;
                    Ok(())
                }
            }
        })
        .await?;

    Ok(())
}
