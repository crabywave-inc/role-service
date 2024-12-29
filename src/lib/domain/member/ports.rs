use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use crate::domain::member::events::MemberCreatedEvent;
use crate::domain::role::entities::model::Role;
use std::future::Future;

pub trait MemberService: Clone + Send + Sync + 'static {
    fn find_member_by_id(
        &self,
        member_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn find_by_user_id(
        &self,
        user_id: &str,
        guild_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn create(
        &self,
        payload: MemberCreatedEvent,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn add_role(
        &self,
        member_id: &str,
        role_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn remove_role(
        &self,
        member_id: &str,
        role_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn get_roles(
        &self,
        member_id: &str,
    ) -> impl Future<Output = Result<Vec<Role>, MemberError>> + Send;
}

pub trait MemberRepository: Clone + Send + Sync + 'static {
    fn find_by_id(
        &self,
        member_id: &str,
    ) -> impl Future<Output = Result<Option<Member>, MemberError>> + Send;
    fn find_by_user_id(
        &self,
        user_id: &str,
        guild_id: &str,
    ) -> impl Future<Output = Result<Option<Member>, MemberError>> + Send;
    fn create(
        &self,
        payload: MemberCreatedEvent,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn add_role(
        &self,
        member_id: &str,
        role_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
    fn remove_role(
        &self,
        member_id: &str,
        role_id: &str,
    ) -> impl Future<Output = Result<Member, MemberError>> + Send;
}
