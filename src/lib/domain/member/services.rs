use std::sync::Arc;

use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use crate::domain::member::events::MemberCreatedEvent;
use crate::domain::member::ports::{MemberRepository, MemberService};
use crate::domain::role::entities::model::Role;
use crate::domain::role::ports::role::RoleService;

#[derive(Debug, Clone)]
pub struct MemberServiceImpl<M, R>
where
    M: MemberRepository,
    R: RoleService,
{
    member_repository: M,
    role_service: Arc<R>,
}

impl<M, R> MemberServiceImpl<M, R>
where
    M: MemberRepository,
    R: RoleService,
{
    pub fn new(member_repository: M, role_service: Arc<R>) -> Self {
        Self {
            member_repository,
            role_service,
        }
    }
}

impl<M, R> MemberService for MemberServiceImpl<M, R>
where
    M: MemberRepository,
    R: RoleService,
{
    async fn find_member_by_id(&self, member_id: &str) -> Result<Member, MemberError> {
        let member = self.member_repository.find_by_id(member_id).await?;

        match member {
            Some(member) => Ok(member),
            None => Err(MemberError::NotFound(format!(
                "Member with id {} not found",
                member_id
            ))),
        }
    }

    async fn find_by_user_id(&self, user_id: &str, guild_id: &str) -> Result<Member, MemberError> {
        let member = self
            .member_repository
            .find_by_user_id(user_id, guild_id)
            .await?;

        match member {
            Some(member) => Ok(member),
            None => Err(MemberError::NotFound(format!(
                "Member with user_id {} and guild_id {} not found",
                user_id, guild_id
            ))),
        }
    }

    async fn create(&self, payload: MemberCreatedEvent) -> Result<Member, MemberError> {
        self.member_repository.create(payload).await
    }

    async fn add_role(&self, member_id: &str, role_id: &str) -> Result<Member, MemberError> {
        self.member_repository.add_role(member_id, role_id).await
    }

    async fn remove_role(&self, member_id: &str, role_id: &str) -> Result<Member, MemberError> {
        self.member_repository.remove_role(member_id, role_id).await
    }

    async fn get_roles(&self, member_id: &str) -> Result<Vec<Role>, MemberError> {
        let member = self.find_member_by_id(member_id).await?;
        let role_ids = member.role_ids;

        let roles = self
            .role_service
            .get_roles(role_ids)
            .await
            .map_err(|_| MemberError::InternalServerError)?;

        Ok(roles)
    }
}
