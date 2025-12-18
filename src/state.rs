use axum::extract::FromRef;

use crate::infra::{PrimaryDb, ReplicaDb};

#[derive(Clone)]
pub struct AppState {
    pub primary: PrimaryDb,
    pub replica: ReplicaDb,
}

impl FromRef<AppState> for PrimaryDb {
    fn from_ref(state: &AppState) -> Self {
        state.primary.clone()
    }
}

impl FromRef<AppState> for ReplicaDb {
    fn from_ref(state: &AppState) -> Self {
        state.replica.clone()
    }
}