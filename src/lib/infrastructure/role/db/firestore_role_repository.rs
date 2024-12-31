use firestore::FirestoreQueryFilter;

use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::{CreateRoleRequest, Role};
use crate::domain::role::ports::role::RoleRepository;
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
    async fn create(&self, guild_id: &str, payload: CreateRoleRequest) -> Result<Role, RoleError> {
        let id = uuid::Uuid::new_v4().to_string();
        let role = Role {
            id,
            name: payload.name.to_string(),
            guild_id: guild_id.to_string(),
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

    async fn find_by_guild_id(&self, guild_id: &str) -> Result<Vec<Role>, RoleError> {
        let roles = self
            .firestore
            .db
            .fluent()
            .select()
            .from("roles")
            .filter(|q| q.for_all([q.field("guild_id").eq(guild_id)]))
            .obj::<Role>()
            .query()
            .await
            .map_err(|_| RoleError::InternalServerError)?;

        Ok(roles)
    }

    async fn find_by_id(&self, role_id: &str) -> Result<Option<Role>, RoleError> {
        let role: Option<Role> = self
            .firestore
            .db
            .fluent()
            .select()
            .by_id_in("roles")
            .obj()
            .one(role_id)
            .await
            .map_err(|_| RoleError::NotFound)?;

        Ok(role)
    }
}
