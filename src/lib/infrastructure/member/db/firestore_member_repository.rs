use std::future::Future;
use std::sync::Arc;
use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use crate::domain::member::ports::MemberRepository;
use crate::infrastructure::db::firestore::Firestore;

#[derive(Debug, Clone)]
pub struct FirestoreMemberRepository {
    firestore: Arc<Firestore>,
}

impl FirestoreMemberRepository {
    pub fn new(firestore: Arc<Firestore>) -> Self {
        Self { firestore }
    }
}

impl MemberRepository for FirestoreMemberRepository {
    async fn find_by_id(&self, member_id: &str) -> Result<Option<Member>, MemberError> {
        let member: Option<Member> = self
            .firestore
            .db
            .fluent()
            .select()
            .by_id_in("members")
            .obj()
            .one(member_id)
            .await
            .map_err(|_| MemberError::NotFound)?;

        Ok(member)
    }
}