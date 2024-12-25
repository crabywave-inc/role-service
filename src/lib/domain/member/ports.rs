use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use std::future::Future;
use crate::domain::member::events::MemberCreatedEvent;

pub trait MemberService: Clone + Send + Sync + 'static {
    fn find_member_by_id(
        &self,
        member_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn create(
        &self,
        payload: MemberCreatedEvent,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
}

pub trait MemberRepository: Clone + Send + Sync + 'static {
    fn find_by_id(
        &self,
        member_id: &str,
    ) -> impl Future<Output = Result<Option<Member>, MemberError>> + Send;
}
