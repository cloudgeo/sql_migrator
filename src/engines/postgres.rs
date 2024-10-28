use std::str::FromStr;

use tokio_postgres::{Client, Config, NoTls};

use super::EngineImpl;

const TABLE_CREATION_STATEMENT: &'static str = "CREATE TABLE IF NOT EXISTS sql_migrations (hash VARCHAR NOT NULL, filename UNIQUE VARCHAR NOT NULL, timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);";

async fn establish_connection(connection_string: &str) -> Client {
    let config = Config::from_str(connection_string).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();

    tokio::spawn(connection);

    return client;
}

pub struct PostgresEngine {
    connection: Client,
}

impl PostgresEngine {
    pub async fn new(connection_string: &str) -> PostgresEngine {
        return PostgresEngine { connection: establish_connection(connection_string).await };
    }
}

#[async_trait::async_trait]
impl EngineImpl for PostgresEngine {
    async fn build_migration_table(&self) -> anyhow::Result<()> {
        println!("Build");
        self.connection.batch_execute(TABLE_CREATION_STATEMENT).await?;
        return Ok(());
    }
    
    
    async fn migrate(&self, file_data: &str) -> anyhow::Result<()> {
        println!("Migrate");
        self.connection.batch_execute(file_data).await?;
        return Ok(());
    }

    async fn hash_exists<T>(&self, val: &T) -> anyhow::Result<bool>
    where T: ToString + std::fmt::Display + Sync {
        println!("Exists");
        let query = "SELECT hash FROM sql_migrations WHERE hash = ($1);"; 
        let row = self.connection.query(query, &[&val.to_string()]).await?;
        
        return Ok(row.len() > 0);
    }

    async fn add_hash<T>(&self, val: &T, filename: &str) -> anyhow::Result<()> 
    where T: ToString + std::fmt::Display + Sync {
        println!("Hash");
        let query = "INSERT INTO sql_migrations (hash, filename) VALUES ($1, $2);"; 
        self.connection.execute(query, &[&val.to_string(), &filename]).await?;

        return Ok(());
    }
}