use std::future::Future;
use crate::domain::role::ports::RoleRepository;
use crate::infrastructure::db::firestore::Firestore;
use std::sync::Arc;
use crate::domain::role::entities::error::RoleError;
use crate::domain::role::entities::model::Role;

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
    async fn create(&self) -> Result<Role, RoleError>{
        todo!()
    }

    async fn find_by_server_id(&self, role_id: &str) -> Result<Vec<Role>, RoleError> {
        todo!()
    }
}
