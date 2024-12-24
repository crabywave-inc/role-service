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

impl RoleRepository for FirestoreRoleRepository {}
