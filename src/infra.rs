use sqlx::MySqlPool;

#[derive(Clone)]
pub struct PrimaryDb(pub MySqlPool);

#[derive(Clone)]
pub struct ReplicaDb(pub MySqlPool);