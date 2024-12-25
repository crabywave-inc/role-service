use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use crate::domain::member::events::MemberCreatedEvent;
use crate::domain::member::ports::{MemberRepository, MemberService};
use std::future::Future;

#[derive(Debug, Clone)]
pub struct MemberServiceImpl<M>
where
    M: MemberRepository,
{
    member_repository: M,
}

impl<M> MemberServiceImpl<M>
where
    M: MemberRepository,
{
    pub fn new(member_repository: M) -> Self {
        Self { member_repository }
    }
}

impl<M> MemberService for MemberServiceImpl<M>
where
    M: MemberRepository,
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

    async fn create(&self, payload: MemberCreatedEvent) -> Result<Member, MemberError> {
        self.member_repository.create(payload).await
    }

    async fn add_role(&self, member_id: &str, role_id: &str) -> Result<Member, MemberError> {
        self.member_repository.add_role(member_id, role_id)
            .await
    }
}
