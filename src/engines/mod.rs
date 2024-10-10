pub mod postgres;

#[async_trait::async_trait]
pub trait EngineImpl {
    async fn build_migration_table(&self) -> anyhow::Result<()>;
    async fn hash_exists<T>(&self, val: &T) -> anyhow::Result<bool> where T: ToString + std::fmt::Display + Sync;
    async fn add_hash<T>(&self, val: &T) -> anyhow::Result<()> where T: ToString + std::fmt::Display + Sync;
    async fn migrate(&self, file_data: &str) -> anyhow::Result<()>;
}