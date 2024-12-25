use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use crate::domain::role::ports::RoleRepository;
use crate::infrastructure::db::firestore::Firestore;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FirestoreRoleRepository {
    firestore: Arc<Firestore>,
}

impl FirestoreRoleRepository {
    pub fn new(firestore: Arc<Firestore>) -> Self {
        Self { firestore }
    }
}

impl RoleRepository for FirestoreRoleRepository {
    async fn create(&self, server_id: &str, payload: CreateRoleRequest) -> Result<Role, RoleError> {
        let id = uuid::Uuid::new_v4().to_string();
        let role = Role {
            id,
            name: payload.name.to_string(),
            server_id: server_id.to_string(),
            color: payload.color,
            position: 0,
            permissions: payload.permissions,
            hoist: payload.hoist,
            mentionable: payload.mentionable,
        };

        self.firestore
            .db
            .fluent()
            .insert()
            .into("roles")
            .document_id(&role.id)
            .object(&role)
            .execute::<()>()
            .await
            .map_err(|e| RoleError::CreateError(e.to_string()))?;

        Ok(role)
    }

    async fn find_by_server_id(&self, _role_id: &str) -> Result<Vec<Role>, RoleError> {
        todo!()
    }
}
