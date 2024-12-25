use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use crate::domain::member::ports::{MemberRepository, MemberService};

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
            None => Err(MemberError::NotFound),
        }
    }
}