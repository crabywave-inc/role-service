use crate::domain::member::entities::error::MemberError;
use crate::domain::member::entities::model::Member;
use crate::domain::member::events::MemberCreatedEvent;
use crate::domain::member::ports::MemberRepository;
use crate::infrastructure::db::firestore::Firestore;
use firestore::struct_path::paths;
use firestore::{FirestoreFieldTransform, FirestoreValue};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use tracing::info;

#[derive(Serialize)]
struct UpdateRoleIds {
    #[serde(rename = "role_ids")]
    role_ids: Vec<String>,
}

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
            .map_err(|e| MemberError::NotFound(e.to_string()))?;

        Ok(member)
    }

    async fn create(&self, payload: MemberCreatedEvent) -> Result<Member, MemberError> {
        let member = Member::from_event(payload);

        self.firestore
            .db
            .fluent()
            .insert()
            .into("members")
            .document_id(&member.id)
            .object(&member)
            .execute::<()>()
            .await
            .map_err(|e| MemberError::CreateError(e.to_string()))?;

        Ok(member)
    }

    async fn add_role(&self, member_id: &str, role_id: &str) -> Result<Member, MemberError> {
        info!("Adding role {} to member {}", role_id, member_id);
        let existing_member = self
            .find_by_id(member_id)
            .await?
            .ok_or(MemberError::NotFound(format!("member {} not found", member_id)))?;

        let mut updated_role_ids = existing_member.role_ids.clone();
        if !updated_role_ids.contains(&role_id.to_string()) {
            updated_role_ids.push(role_id.to_string());
        }

        let update_payload = UpdateRoleIds {
            role_ids: vec![role_id.to_string()],
        };

        let member = self
            .firestore
            .db
            .fluent()
            .update()
            .in_col("members")
            .document_id(member_id)
            .object(&Member {
                role_ids: updated_role_ids,
                ..existing_member.clone()
            })
            .execute::<Member>()
            .await
            .map_err(|e| MemberError::UpdateError(e.to_string()))?;

        Ok(member)
    }

    async fn remove_role(&self, member_id: &str, role_id: &str) -> Result<Member, MemberError> {
        todo!()
    }
}
